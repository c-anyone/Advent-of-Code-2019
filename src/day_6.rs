/*
--- Day 6: Universal Orbit Map ---

You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).

Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:

                  \
                   \
                    |
                    |
AAA--> o            o <--BBB
                    |
                    |
                   /
                  /

In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".

Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.

Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.

For example, suppose you have the following map:

COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L

Visually, the above map of orbits looks like this:

        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I

In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.

Here, we can count the total number of orbits as follows:

    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
    COM orbits nothing.

The total number of direct and indirect orbits in this example is 42.

What is the total number of direct and indirect orbits in your map data?
*/

use itertools::Itertools;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let result = get_gravity_map_sum("input_day6.txt");
    println!("Result is {}", result);
    let result = get_gravity_map_sum("input_day6_test.txt");
    println!("Result is {}", result);
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    head: String,
    children: Option<RefCell<Vec<Node>>>,
}

#[derive(Debug, Clone)]
struct Tree {
    nodes: HashMap<String, Node>,
    head: Option<Box<Node>>,
}

impl Node {
    fn add_children(&mut self, node: Node) {
        if self.children.is_none() {
            self.children = Some(RefCell::new(Vec::new()));
        }
        let mut x: RefMut<_> = self.children.as_ref().unwrap().borrow_mut();
        x.push(node);
    }

    fn new(id: &str, head: &str, children: Option<RefCell<Vec<Node>>>) -> Self {
        Node {
            id: id.to_string(),
            head: head.to_string(),
            children: children,
        }
    }
}

impl TryFrom<&str> for Node {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, id) = parse_line(value).ok_or("error parsing line")?;
        Ok(Node {
            id: id,
            head: head,
            children: None,
        })
    }
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            head: None,
        }
    }

    fn insert(&mut self, node: Node) -> Option<()> {
        if !self.nodes.contains_key(&node.head) {
            let head_node = Node::new(&node.id, "", None);
            self.nodes.insert(head_node.id.clone(), head_node);
        }
        if self.nodes.contains_key(&node.id) {
            self.nodes.insert(node.id.clone(), node.to_owned());
        }

        Some(())
    }
}

fn construct_tree(filename: &str) {
    let reader = BufReader::new(File::open(filename).expect("File not found"));

    let mut tree = Tree::new();

    reader.lines().enumerate().for_each(|(n, x)| {
        let x = x.unwrap();
        let mut node = Node::try_from(x.as_ref()).unwrap();
        tree.insert(node);
    });
    // for line in reader.lines()
}

pub fn get_gravity_map_sum(filename: &str) -> u32 {
    let reader = BufReader::new(File::open(filename).expect("File not found"));
    let iter: Vec<(String, String)> = reader
        .lines()
        .filter_map(|x| parse_line(&x.unwrap()).to_owned())
        .collect();

    let mut hm: HashMap<String, u32> = HashMap::new();
    for (grav, id) in iter {
        // let val = hm.get_mut()
        let dist = match hm.get(&grav) {
            Some(y) => *y + 1,
            None => 1,
        };
        hm.insert(id.to_owned(), dist);
    }
    hm.values().sum()
}

fn parse_line(line: &str) -> Option<(String, String)> /* -> (&str, &str) */ {
    match line.splitn(2, ')').collect_tuple() {
        Some((x, y)) => Some((x.to_string(), y.to_string())),
        None => return None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sum_testinput() {
        let result = get_gravity_map_sum("input_day6_test.txt");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_sum_puzzle() {
        let result = get_gravity_map_sum("input_day6.txt");
        assert_eq!(result > 2190, true);
    }
}
