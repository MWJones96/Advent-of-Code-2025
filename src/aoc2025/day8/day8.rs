use std::cmp::{Reverse, min};
use std::collections::{BinaryHeap, HashMap, HashSet};

use ordered_float::OrderedFloat;

#[derive(Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn dist(&self, other: &JunctionBox) -> f64 {
        let dist_sq = self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2);

        (dist_sq as f64).sqrt()
    }
}

fn find(parents: &Vec<usize>, x: usize) -> usize {
    if parents[x] != x {
        return find(parents, parents[x]);
    }
    x
}

fn union(parents: &mut Vec<usize>, x: usize, y: usize) {
    let parent_y = find(parents, y);
    let parent_x = find(parents, x);
    parents[parent_y] = parent_x;
}

fn part1() {
    let input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
    let input: Vec<Vec<u64>> = input
        .iter()
        .map(|s| (*s).split(",").map(|s2| s2.parse().unwrap()).collect())
        .collect();
    let mut junction_boxes = vec![];
    let mut heap = BinaryHeap::new();
    let mut parents = vec![];
    for xyz in input {
        let junction_box = JunctionBox {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        };
        junction_boxes.push(junction_box);
    }
    for i in 0..junction_boxes.len() {
        parents.push(i);
        for j in (i + 1)..junction_boxes.len() {
            let dist = junction_boxes[i].dist(&junction_boxes[j]);
            heap.push((Reverse(OrderedFloat(dist)), i, j));
        }
    }
    for _ in 0..min(1000, heap.len()) {
        let (_, i, j) = heap.pop().unwrap();
        union(&mut parents, i, j);
    }
    for i in 0..parents.len() {
        parents[i] = find(&parents, i);
    }
    let mut freq = HashMap::new();
    for num in parents {
        *freq.entry(num).or_insert(0 as u64) += 1;
    }
    let mut vals: Vec<u64> = freq.values().cloned().collect();
    vals.sort_by(|a, b| b.cmp(a));

    println!(
        "(Part 1) Product of three largest circuits: {}",
        vals[0..3].iter().copied().reduce(|a, b| a * b).unwrap()
    );
}

fn part2() {
    let _input = include_str!("input.txt");
    // TODO: implement Part 2
    println!("(Part 2) not implemented yet");
}

pub fn day8() {
    println!("---- DAY 8 ----");
    part1();
    part2();
    println!("");
}
