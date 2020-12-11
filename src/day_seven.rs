use std::{str::CharIndices, iter::Peekable};

use crate::errors::AppResult;

const INPUT: &'static str = include_str!("../data/bag-rules.txt");

#[derive(Debug)]
struct Tree<T> where T: PartialEq {
  arena: Vec<Node<T>>,
}

impl<T> Tree<T> where T: PartialEq {
  pub fn new() -> Self {
    Self { arena: vec![] }
  }

  pub fn find(&self, value: T) -> Option<usize> {
    self.arena.iter().find(|node| node.value == value).map(|node| node.id)
  }
}

#[derive(Debug)]
struct Node<T> where T: PartialEq {
  id: usize,
  parent: Option<usize>,
  children: Vec<usize>,
  value: T,
}

impl<T> Node<T> where T: PartialEq {
  pub fn new(id:usize, value: T) -> Self {
    Self {
      id,
      parent: None,
      children: vec![],
      value,
    }
  }
}

#[derive(Debug, PartialEq)]
struct Bag {
  name: String,
  count: u32,
}

#[derive(Debug)]
enum Token<'a> {
  Identifer(&'a str),
  Integer(u32),
  Unknown(&'a str),
  Comma,
  Dot,
  Eof,
}

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
      },
      Some((position, c)) if c.is_alphabetic() => {
        self.position = position;
        self.offset = self.consume_while(char::is_alphabetic);

        Some(Token::Identifer(&self.input[self.position..=self.offset]))
      },
      Some((position, c)) if c.is_numeric() => {
        self.position = position;
        self.offset = self.consume_while(char::is_numeric);
        let value = self.input[self.position..=self.offset].parse::<i32>().expect("Unable to parse number as int");

        Some(Token::Integer(value))
      },
      Some((position, ',')) => {
        self.position = position;
        self.offset = position;

        Some(Token::Comma)
      },
      Some((position, '.')) => {
        self.position = position;
        self.offset = position;

        Some(Token::Dot)
      },
      Some((index, _)) => Some(Token::Unknown(&self.input[index..])),
      None => None,
    }
  }

  fn consume_while<F>(&mut self, predicate: F) -> usize
    where F: Fn(char) -> bool {
      let mut offset = self.position;

      while let Some((index, value)) = self.iter.peek() {
        match predicate(*value) {
          true => {
            offset = *index;
            self.iter.next();
          },
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

fn parse_bag(lexer: &mut Lexer) -> Bag {
  let name = lexer
    .take_while(|token| match token {
      Token::Identifer("bag") => false,
      Token::Identifer(_) => true,
      _ => false,
    })
    .skip(1)
    .map(|token| match token {
      Token::Identifer(val) => val,
      _ => "",
    })
    .collect::<Vec<_>>()
    .join(" ");

  Bag {
    name,
    count: 1,
  }
}

fn parse_bag_with_count(lexer: &mut Lexer) -> Bag {
  let count = match lexer.next() {
    Some(Token::Integer(val)) => val,
    _ => panic!("Unexpected token"),
  };

  let Bag { name, .. } = parse_bag(lexer);

  Bag {
    name,
    count,
  }
}

// grammar
//
// name = <STRING> bags
// bag_count = <NUMBER> <name>
// <name> contain <bag_count>[, ...bag_count].
fn parse_rules(input: &str) -> Tree<Bag> {
  let lexer = &mut Lexer::new(input);

  let container_bag = parse_bag(lexer);
  println!("{:?}", container_bag);

  let mut inner_bags = vec![parse_bag_with_count(lexer)];

  while let Some(token) = lexer.next() {
    match token {
      "" => inner_bags.push(parse_bag_with_count(lexer.skip(1))),
      _ => panic!(format!("Unexpected token \"{}\"", token)),
    }
  }

  println!("{:?}", container_bag);
  println!("{:?}", inner_bags);

  Tree::new()
}

pub fn run() -> AppResult<()> {
  let input = "bri京ght indigo bags contain 4 shiny turquoise bags, 3 wavy yellow bags.\n";
  // let input = "b ";
  println!("{}", input);
  let mut lexer = Lexer::new(input);
  while let Some(token) = lexer.next() {
    println!("{:?}", token);
  }
  // parse_rules(inp);
  // parse_rules(INPUT);

  Ok(())
}
