use ipnet::Ipv4Net;
use ptree::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

fn route_ipv4_add(ptree: &mut Ptree<Ipv4Net, i32>, str: &str, data: i32) {
    let prefix: Ipv4Net = str.parse().unwrap();
    ptree.add(&prefix, data);
}

fn route_ipv4_delete(ptree: &mut Ptree<Ipv4Net, i32>, str: &str) {
    let prefix: Ipv4Net = str.parse().unwrap();
    ptree.delete(&prefix);
}

#[test]
fn ipv4_route_random1() {
    let now = time::Instant::now();

    let mut top = Ptree::new();

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        route_ipv4_add(&mut top, &line, 0);
    }
    assert_eq!(top.iter().count(), 569770);

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        route_ipv4_delete(&mut top, &line);
    }

    assert_eq!(top.iter().count(), 0);

    println!("Elapsed {:?}", now.elapsed());
}
