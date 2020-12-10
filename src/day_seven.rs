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

struct Tokenizer<'a> {
  data: Vec<&'a str>,
  location: usize,
}

impl<'a> Tokenizer<'a> {
  pub fn new(data: Vec<&'a str>) -> Self {
    Self {
      data,
      location: 0,
    }
  }

  pub fn current(&self) -> Option<&'a str> {
    self.data.get(self.location).map(|&value| value)
  }

  pub fn next(&mut self) -> Option<&'a str> {
    self.peek_next().map(|value| {
      self.location += 1;
      value
    })
  }

  pub fn peek_next(&mut self) -> Option<&'a str> {
    self.data.get(self.location + 1).map(|&value| value)
  }

  pub fn take_while<F>(&mut self, predicate: F) -> Vec<&'a str>
    where F: Fn(&'a str) -> bool {
      let start_index = self.location;
      let mut end_index = start_index + 1;

      while let Some(next) = self.peek_next() {
        println!("NEXT {}", next);
        match predicate(next) {
          true => end_index += 1,
          false => break,
        }

        self.next();
      }

      self.location = end_index;
      self.data[start_index..end_index].to_vec()
  }

  pub fn skip(&mut self, count: usize) -> &mut Self {
    let new_location = self.location + count;
    if new_location >= self.data.len() - 1 {
      self.location = new_location;
    } else {
      self.location = new_location;
    }

    self
  }
}

fn parse_bag(tokenizer: &mut Tokenizer) -> Bag {
  Bag {
    name: tokenizer.take_while(|token| token != "bags").join(" "),
    count: 1,
  }
}

fn parse_bag_with_count(tokenizer: &mut Tokenizer) -> Bag {
  let count = tokenizer.current().expect("Missing bag count").parse::<u32>().expect("Bag count is not a number");
  let Bag { name, .. } = parse_bag(tokenizer);

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
  let a = input.split("\n").map(|line| {
    println!("{}", line);
    println!("{:?}", line.split(|c| c == ' ' || c == '.' || c == ',').collect::<Vec<_>>());
    let tokenizer = &mut Tokenizer::new(line.split(|c| c == ' ' || c == '.' || c == ',').collect::<Vec<_>>());

    let container_bag = parse_bag(tokenizer);
    println!("{:?}", container_bag);

    let mut inner_bags = vec![parse_bag_with_count(tokenizer.skip(2))];

    while let Some(token) = tokenizer.next() {
      match token {
        "" => inner_bags.push(parse_bag_with_count(tokenizer.skip(1))),
        _ => panic!(format!("Unexpected token \"{}\"", token)),
      }
    }

    println!("{:?}", container_bag);
    println!("{:?}", inner_bags);
  })
  .collect::<Vec<_>>();

  Tree::new()
}

pub fn run() -> AppResult<()> {
  let inp = "bright indigo bags contain 4 shiny turquoise bags, 3 wavy yellow bags.\n";
  parse_rules(inp);
  // parse_rules(INPUT);

  Ok(())
}
