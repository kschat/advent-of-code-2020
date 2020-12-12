use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter::Peekable,
    str::CharIndices,
};

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/bag-rules.txt");
#[derive(Debug)]
enum Token<'a> {
    Identifer(&'a str),
    Integer(i32),
    Unknown(&'a str),
    Comma,
    Dot,
}

// grammar
//
// name = <STRING> bags
// bag_count = <NUMBER> <name>
// empty_bag = no other bags
// <name> contain <<bag_count>[, ...bag_count]|<empty_bag>>.
struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    position: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
            position: 0,
            offset: 0,
        }
    }

    fn consume_while<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut offset = self.position;

        while let Some((index, value)) = self.iter.peek() {
            match predicate(*value) {
                true => {
                    offset = *index;
                    self.iter.next();
                }
                false => break,
            }
        }

        offset
    }

    fn consume_identifier(&mut self, position: usize) -> Token<'a> {
        self.position = position;
        self.offset = self.consume_while(char::is_alphabetic);

        Token::Identifer(&self.input[self.position..=self.offset])
    }

    fn consume_integer(&mut self, position: usize) -> Token<'a> {
        self.position = position;
        self.offset = self.consume_while(char::is_numeric);
        let value = self.input[self.position..=self.offset]
            .parse::<i32>()
            .expect("Unable to parse number as int");

        Token::Integer(value)
    }

    fn consume_comma(&mut self, position: usize) -> Token<'a> {
        self.position = position;
        self.offset = position;

        Token::Comma
    }

    fn consume_dot(&mut self, position: usize) -> Token<'a> {
        self.position = position;
        self.offset = position;

        Token::Dot
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((position, c)) if c.is_whitespace() => {
                self.position = position;
                self.offset = self.consume_while(char::is_whitespace);

                self.next()
            }
            Some((position, c)) if c.is_alphabetic() => Some(self.consume_identifier(position)),
            Some((position, c)) if c.is_numeric() => Some(self.consume_integer(position)),
            Some((position, ',')) => Some(self.consume_comma(position)),
            Some((position, '.')) => Some(self.consume_dot(position)),
            Some((index, _)) => Some(Token::Unknown(&self.input[index..])),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Bag {
    name: String,
    count: i32,
}

fn parse_bag(lexer: &mut Peekable<Lexer>) -> Bag {
    let name = lexer
        .take_while(|token| match token {
            Token::Identifer("bag") | Token::Identifer("bags") => false,
            Token::Identifer(_) => true,
            _ => false,
        })
        .map(|token| match token {
            Token::Identifer(val) => val,
            _ => "",
        })
        .collect::<Vec<_>>()
        .join(" ");

    Bag { name, count: 1 }
}

fn parse_bag_with_count(lexer: &mut Peekable<Lexer>) -> Bag {
    let count = match lexer.next() {
        Some(Token::Integer(val)) => val,
        Some(token) => panic!(format!("Unexpected token \"{:?}\"", token)),
        None => panic!("Unexpected end of input"),
    };

    Bag { name: parse_bag(lexer).name, count }
}

struct TableNode {
    parents: Vec<Bag>,
    children: Vec<Bag>,
}

impl TableNode {
    pub fn new() -> Self {
        Self { parents: vec![], children: vec![] }
    }
}

type LookupTable = HashMap<String, TableNode>;

fn parse_rules(lexer: &mut Peekable<Lexer>) -> LookupTable {
    let mut lookup_table = LookupTable::new();

    while let Some(token) = lexer.peek() {
        match token {
            Token::Identifer(_) => {
                let (container, children) = parse_rule(lexer);
                let children = children.unwrap_or_else(|| vec![]);

                lookup_table
                    .entry(container.clone().name)
                    .or_insert(TableNode::new())
                    .children
                    .extend(children.clone());

                for child in children {
                    lookup_table
                        .entry(child.name)
                        .or_insert(TableNode::new())
                        .parents
                        .push(container.clone());
                }
            }
            _ => panic!(format!("Unexpected token \"{:?}\"", token)),
        }
    }

    lookup_table
}

fn parse_rule(lexer: &mut Peekable<Lexer>) -> (Bag, Option<Vec<Bag>>) {
    let container_bag = parse_bag(lexer);

    match lexer.next() {
        Some(Token::Identifer("contain")) => (),
        Some(token) => panic!(format!("Expected 'contain', found \"{:?}\"", token)),
        None => panic!("Unexpected end of input"),
    };

    match lexer.peek() {
        Some(Token::Identifer("no")) => {
            lexer.skip(3).next();
            return (container_bag, None);
        }
        _ => (),
    };

    let mut inner_bags = vec![parse_bag_with_count(lexer)];
    while let Some(token) = lexer.next() {
        match token {
            Token::Comma => inner_bags.push(parse_bag_with_count(lexer)),
            Token::Dot => break,
            _ => panic!(format!("Unexpected token \"{:?}\"", token)),
        }
    }

    (container_bag, Some(inner_bags))
}

fn get_parents<'a>(
    bag_name: &str,
    lookup_table: &'a LookupTable,
) -> HashSet<&'a str> {
    let parents = match lookup_table.get(bag_name) {
        Some(value) => &value.parents,
        None => return HashSet::new(),
    };

    parents.iter().fold(HashSet::new(), |mut set, bag| {
        set.insert(&bag.name);
        set.extend(get_parents(&bag.name, lookup_table));
        set
    })
}

fn get_required_bag_count(
    bag_name: &str,
    lookup_table: &LookupTable,
) -> i32 {
    let children = match lookup_table.get(bag_name) {
        Some(value) => &value.children,
        None => return 0,
    };

    children.iter().fold(0, |acc, bag| {
        acc + bag.count + (bag.count * get_required_bag_count(&bag.name, lookup_table))
    })
}

pub fn run() -> AppResult<()> {
    let mut lexer = Lexer::new(INPUT).peekable();
    let hash = parse_rules(&mut lexer);

    println!("Part 1: \"{}\"", get_parents("shiny gold", &hash).len());
    println!("Part 2: \"{}\"", get_required_bag_count("shiny gold", &hash));

    Ok(())
}
