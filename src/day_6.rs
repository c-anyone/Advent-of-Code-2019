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
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let result = day_6_main("input_day6_test.txt");
    println!("Result is {}", result);
    let result = day_6_main("input_day6.txt");
    println!("Result is {}", result);
}

pub fn day_6_main(filename: &str) -> u32 {
    let reader = BufReader::new(File::open(filename).expect("File not found"));
    let hm = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|x| parse_line(&x).unwrap())
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    for mut body in hm.keys() {
        // loop {
        while let Some(parent) = hm.get(body) {
            count += 1;
            body = parent;
        }
    }
    
    let mut object = match hm.get("YOU"){
        Some(x) => x,
        None => return count,
    };

    let mut my_ancestors = HashMap::new();


    let mut anc = 1;
    while let Some(parent) = hm.get(object) {
        my_ancestors.insert(parent, anc);
        object = parent;
        anc += 1;
    }

    object = hm.get("SAN").unwrap();

    let mut dist = 1;
    while let Some(parent) = hm.get(object) {
        if let Some(ancestor) = my_ancestors.get(parent) {
            println!("Distance to Santa is {}", dist + ancestor);
            break;
        }
        dist +=1;
        object = parent;
    }


    count
}

// fn get_orbit_count_of(orbit: &str, )

fn parse_line(line: &str) -> Option<(String, String)> /* -> (&str, &str) */ {
    match line.splitn(2, ')').collect_tuple() {
        Some((y, x)) => Some((x.to_string(), y.to_string())),
        None => return None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sum_testinput() {
        let result = day_6_main("input_day6_test.txt");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_sum_puzzle() {
        let result = day_6_main("input_day6.txt");
        assert_eq!(result > 2190, true);
    }
}
