use std::{fmt::Debug, iter::Peekable, str::CharIndices};
use uuid::Uuid;

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/bag-rules.txt");

#[derive(Debug)]
struct Tree<T>
where
    T: PartialEq,
    T: Debug,
{
    arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq,
    T: Debug,
{
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn node(&mut self, value: T) -> usize {
        match self.arena.iter().find(|node| node.value == value) {
            Some(node) => node.id,
            None => {
                let id = self.arena.len();
                self.arena.push(Node::new(id, value));
                id
            }
        }
    }

    pub fn insert(&mut self, parent: T, children: Vec<T>) -> (usize, Vec<usize>) {
        let parent_id = self.node(parent);
        let children_ids = children
            .into_iter()
            .map(|child| {
                let id = self.node(child);
                match self.arena[id].parents.iter().find(|&&node| node == parent_id) {
                    Some(_) => (),
                    None => {
                        self.arena[id].parents.push(parent_id);
                        self.arena[parent_id].children.push(id);
                    }
                }

                // match self.arena[id].parents {
                //     Some(pid) => panic!(format!(
                //         "Node \"{:?}\" already has parent \"{:?}\"",
                //         self.arena[id].value, self.arena[pid].value
                //     )),
                //     None => {
                //         self.arena[id].parent = Some(parent_id);
                //         self.arena[parent_id].children.push(id);
                //     }
                // }

                id
            })
            .collect::<Vec<_>>();

        (parent_id, children_ids)
    }

    pub fn filter<F>(&self, predicate: F) -> Vec<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.arena
            .iter()
            .filter_map(|x| match predicate(&x.value) {
                true => Some(x.id),
                false => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn node_depth(&self, id: usize) -> usize {
        println!("DEPTH {:?}", self.arena[id]);
        self.arena[id].parents.iter().fold(0, |acc, &parent| {
            acc + 1 + self.node_depth(parent)
        })

        // match self.arena[id].parents {
        //     Some(id) => 1 + self.node_depth(id),
        //     None => 0,
        // }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    id: usize,
    // parent: Option<usize>,
    parents: Vec<usize>,
    children: Vec<usize>,
    value: T,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(id: usize, value: T) -> Self {
        Self {
            id,
            // parent: None,
            parents: vec![],
            children: vec![],
            value,
        }
    }
}

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

    fn next_token(&mut self) -> Option<Token<'a>> {
        match self.iter.next() {
            Some((position, c)) if c.is_whitespace() => {
                self.position = position;
                self.offset = self.consume_while(char::is_whitespace);

                self.next_token()
            }
            Some((position, c)) if c.is_alphabetic() => {
                self.position = position;
                self.offset = self.consume_while(char::is_alphabetic);

                Some(Token::Identifer(&self.input[self.position..=self.offset]))
            }
            Some((position, c)) if c.is_numeric() => {
                self.position = position;
                self.offset = self.consume_while(char::is_numeric);
                let value = self.input[self.position..=self.offset]
                    .parse::<i32>()
                    .expect("Unable to parse number as int");

                Some(Token::Integer(value))
            }
            Some((position, ',')) => {
                self.position = position;
                self.offset = position;

                Some(Token::Comma)
            }
            Some((position, '.')) => {
                self.position = position;
                self.offset = position;

                Some(Token::Dot)
            }
            Some((index, _)) => Some(Token::Unknown(&self.input[index..])),
            None => None,
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug, PartialEq)]
struct Bag {
    id: String,
    name: String,
    count: i32,
}

impl Bag {
    pub fn new(name: String, count: i32) -> Self {
        Self {
            id: "".to_string(),
            // id: Uuid::new_v4().to_hyphenated().to_string(),
            name,
            count,
        }
    }
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

    Bag::new(name, 1)
}

fn parse_bag_with_count(lexer: &mut Peekable<Lexer>) -> Bag {
    let count = match lexer.next() {
        Some(Token::Integer(val)) => val,
        Some(token) => panic!(format!("Unexpected token \"{:?}\"", token)),
        None => panic!("Unexpected end of input"),
    };

    Bag::new(parse_bag(lexer).name, 1)
}

fn parse_rules(lexer: &mut Peekable<Lexer>) -> Tree<Bag> {
    let mut bags = Tree::new();
    while let Some(token) = lexer.peek() {
        match token {
            Token::Identifer(_) => {
                let (container, inner_bags) = parse_rule(lexer);
                bags.insert(container, inner_bags.unwrap_or_else(|| vec![]));
                // let (container, children) = parse_rule(lexer);
                // for c in children.unwrap_or_else(|| vec![]) {
                //     bags.insert(c, vec![container.clone()]);
                // }
            }
            _ => panic!(format!("Unexpected token \"{:?}\"", token)),
        }
    }

    bags
}

fn parse_rule(lexer: &mut Peekable<Lexer>) -> (Bag, Option<Vec<Bag>>) {
    let container_bag = parse_bag(lexer);

    println!("{:?}", container_bag);

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

    println!("{:?}", inner_bags);

    (container_bag, Some(inner_bags))
}

pub fn run() -> AppResult<()> {
    let mut lexer = Lexer::new(INPUT).peekable();
    let bags = parse_rules(&mut lexer);
    let targets = bags.filter(|bag| bag.name == "shiny gold");
    println!("Target {:?}", targets);

    println!("Depth {}", targets.iter().fold(0, |acc, &target| acc + bags.node_depth(target)));

    Ok(())
}
