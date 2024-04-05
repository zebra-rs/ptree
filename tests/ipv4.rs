use ipnet::Ipv4Net;
use ptree::*;

fn lookup_assert(top: &Ptree<Ipv4Net, i32>, addr: &str, route: &str) {
    let prefix: Ipv4Net = addr.parse().unwrap();
    let iter = top.lookup(&prefix);
    let n = iter.node;
    // let n = top.route_ipv4_lookup(addr);
    let p: Ipv4Net = route.parse().unwrap();
    assert_eq!(n.unwrap().prefix, p);
}

fn lookup_assert_none(top: &Ptree<Ipv4Net, i32>, addr: &str) {
    let prefix: Ipv4Net = addr.parse().unwrap();
    let iter = top.lookup(&prefix);
    let n = iter.node;
    // let n = top.route_ipv4_lookup(addr);
    assert!(n.is_none());
}

fn lookup_test(top: &Ptree<Ipv4Net, i32>) {
    lookup_assert(top, "10.0.0.0/32", "10.0.0.0/32");
    lookup_assert(top, "10.0.0.1/32", "10.0.0.0/31");
    lookup_assert(top, "10.0.0.2/32", "10.0.0.0/30");
    lookup_assert(top, "10.0.0.3/32", "10.0.0.0/30");

    lookup_assert(top, "10.0.0.4/32", "10.0.0.0/29");
    lookup_assert(top, "10.0.0.7/32", "10.0.0.0/29");
    lookup_assert(top, "10.0.0.8/32", "10.0.0.0/28");
    lookup_assert(top, "10.0.0.15/32", "10.0.0.0/28");
    lookup_assert(top, "10.0.0.0/28", "10.0.0.0/28");

    lookup_assert_none(top, "10.0.0.16/32");
    lookup_assert_none(top, "10.0.0.255/32");
    lookup_assert_none(top, "0.0.0.0/0");
}

fn route_ipv4_add(ptree: &mut Ptree<Ipv4Net, i32>, str: &str, data: i32) {
    let prefix: Ipv4Net = str.parse().unwrap();
    ptree.add(&prefix, data);
}

fn route_ipv4_delete(ptree: &mut Ptree<Ipv4Net, i32>, str: &str) {
    let prefix: Ipv4Net = str.parse().unwrap();
    ptree.delete(&prefix);
}

#[test]
fn ipv4_lookup_reverse_test() {
    let mut top = Ptree::<Ipv4Net, i32>::new();

    // 10.0.0.0/{28..32}
    route_ipv4_add(&mut top, "10.0.0.0/32", 32);
    route_ipv4_add(&mut top, "10.0.0.0/31", 31);
    route_ipv4_add(&mut top, "10.0.0.0/30", 30);
    route_ipv4_add(&mut top, "10.0.0.0/29", 29);
    route_ipv4_add(&mut top, "10.0.0.0/28", 28);

    lookup_test(&mut top);
}

#[test]
fn ipv4_iter_count() {
    let mut top = Ptree::<Ipv4Net, i32>::new();

    route_ipv4_add(&mut top, "0.0.0.0/0", 0);
    route_ipv4_add(&mut top, "0.0.0.0/1", 1);
    route_ipv4_add(&mut top, "128.0.0.0/1", 1);

    route_ipv4_add(&mut top, "0.0.0.0/2", 2);
    route_ipv4_add(&mut top, "64.0.0.0/2", 2);
    route_ipv4_add(&mut top, "128.0.0.0/2", 2);
    route_ipv4_add(&mut top, "192.0.0.0/2", 2);

    route_ipv4_add(&mut top, "0.0.0.0/3", 3);
    route_ipv4_add(&mut top, "32.0.0.0/3", 3);
    route_ipv4_add(&mut top, "64.0.0.0/3", 3);
    route_ipv4_add(&mut top, "96.0.0.0/3", 3);
    route_ipv4_add(&mut top, "128.0.0.0/3", 3);
    route_ipv4_add(&mut top, "160.0.0.0/3", 3);
    route_ipv4_add(&mut top, "192.0.0.0/3", 3);
    route_ipv4_add(&mut top, "224.0.0.0/3", 3);

    route_ipv4_add(&mut top, "0.0.0.0/4", 4);
    route_ipv4_add(&mut top, "32.0.0.0/4", 4);
    route_ipv4_add(&mut top, "64.0.0.0/4", 4);
    route_ipv4_add(&mut top, "96.0.0.0/4", 4);
    route_ipv4_add(&mut top, "128.0.0.0/4", 4);
    route_ipv4_add(&mut top, "160.0.0.0/4", 4);
    route_ipv4_add(&mut top, "192.0.0.0/4", 4);
    route_ipv4_add(&mut top, "224.0.0.0/4", 4);
    route_ipv4_add(&mut top, "16.0.0.0/4", 4);
    route_ipv4_add(&mut top, "48.0.0.0/4", 4);
    route_ipv4_add(&mut top, "89.0.0.0/4", 4);
    route_ipv4_add(&mut top, "112.0.0.0/4", 4);
    route_ipv4_add(&mut top, "144.0.0.0/4", 4);
    route_ipv4_add(&mut top, "176.0.0.0/4", 4);
    route_ipv4_add(&mut top, "208.0.0.0/4", 4);
    route_ipv4_add(&mut top, "240.0.0.0/4", 4);

    assert_eq!(top.iter().count(), 31);
}

#[test]
fn ipv4_iter_count_delete() {
    let mut top = Ptree::<Ipv4Net, i32>::new();

    route_ipv4_add(&mut top, "0.0.0.0/0", 0);
    route_ipv4_add(&mut top, "0.0.0.0/1", 1);
    route_ipv4_add(&mut top, "128.0.0.0/1", 1);

    route_ipv4_add(&mut top, "0.0.0.0/2", 2);
    route_ipv4_add(&mut top, "64.0.0.0/2", 2);
    route_ipv4_add(&mut top, "128.0.0.0/2", 2);
    route_ipv4_add(&mut top, "192.0.0.0/2", 2);

    route_ipv4_add(&mut top, "0.0.0.0/3", 3);
    route_ipv4_add(&mut top, "32.0.0.0/3", 3);
    route_ipv4_add(&mut top, "64.0.0.0/3", 3);
    route_ipv4_add(&mut top, "96.0.0.0/3", 3);
    route_ipv4_add(&mut top, "128.0.0.0/3", 3);
    route_ipv4_add(&mut top, "160.0.0.0/3", 3);
    route_ipv4_add(&mut top, "192.0.0.0/3", 3);
    route_ipv4_add(&mut top, "224.0.0.0/3", 3);

    route_ipv4_add(&mut top, "0.0.0.0/4", 4);
    route_ipv4_add(&mut top, "32.0.0.0/4", 4);
    route_ipv4_add(&mut top, "64.0.0.0/4", 4);
    route_ipv4_add(&mut top, "96.0.0.0/4", 4);
    route_ipv4_add(&mut top, "128.0.0.0/4", 4);
    route_ipv4_add(&mut top, "160.0.0.0/4", 4);
    route_ipv4_add(&mut top, "192.0.0.0/4", 4);
    route_ipv4_add(&mut top, "224.0.0.0/4", 4);
    route_ipv4_add(&mut top, "16.0.0.0/4", 4);
    route_ipv4_add(&mut top, "48.0.0.0/4", 4);
    route_ipv4_add(&mut top, "89.0.0.0/4", 4);
    route_ipv4_add(&mut top, "112.0.0.0/4", 4);
    route_ipv4_add(&mut top, "144.0.0.0/4", 4);
    route_ipv4_add(&mut top, "176.0.0.0/4", 4);
    route_ipv4_add(&mut top, "208.0.0.0/4", 4);
    route_ipv4_add(&mut top, "240.0.0.0/4", 4);

    route_ipv4_delete(&mut top, "0.0.0.0/0");
    route_ipv4_delete(&mut top, "0.0.0.0/1");
    route_ipv4_delete(&mut top, "128.0.0.0/1");

    route_ipv4_delete(&mut top, "0.0.0.0/2");
    route_ipv4_delete(&mut top, "64.0.0.0/2");
    route_ipv4_delete(&mut top, "128.0.0.0/2");
    route_ipv4_delete(&mut top, "192.0.0.0/2");

    route_ipv4_delete(&mut top, "0.0.0.0/3");
    route_ipv4_delete(&mut top, "32.0.0.0/3");
    route_ipv4_delete(&mut top, "64.0.0.0/3");
    route_ipv4_delete(&mut top, "96.0.0.0/3");
    route_ipv4_delete(&mut top, "128.0.0.0/3");
    route_ipv4_delete(&mut top, "160.0.0.0/3");
    route_ipv4_delete(&mut top, "192.0.0.0/3");
    route_ipv4_delete(&mut top, "224.0.0.0/3");

    route_ipv4_delete(&mut top, "0.0.0.0/4");
    route_ipv4_delete(&mut top, "32.0.0.0/4");
    route_ipv4_delete(&mut top, "64.0.0.0/4");
    route_ipv4_delete(&mut top, "96.0.0.0/4");
    route_ipv4_delete(&mut top, "128.0.0.0/4");
    route_ipv4_delete(&mut top, "160.0.0.0/4");
    route_ipv4_delete(&mut top, "192.0.0.0/4");
    route_ipv4_delete(&mut top, "224.0.0.0/4");
    route_ipv4_delete(&mut top, "16.0.0.0/4");
    route_ipv4_delete(&mut top, "48.0.0.0/4");
    route_ipv4_delete(&mut top, "89.0.0.0/4");
    route_ipv4_delete(&mut top, "112.0.0.0/4");
    route_ipv4_delete(&mut top, "144.0.0.0/4");
    route_ipv4_delete(&mut top, "176.0.0.0/4");
    route_ipv4_delete(&mut top, "208.0.0.0/4");
    route_ipv4_delete(&mut top, "240.0.0.0/4");

    assert_eq!(top.iter().count(), 0);
}

#[test]
fn ipv4_delete_default() {
    let mut top = Ptree::<Ipv4Net, i32>::new();

    route_ipv4_add(&mut top, "0.0.0.0/0", 0);
    assert_eq!(top.iter().count(), 1);

    route_ipv4_delete(&mut top, "0.0.0.0/0");
    assert_eq!(top.iter().count(), 0);
}

#[test]
fn ipv4_delete_table_default() {
    let mut top = Ptree::<Ipv4Net, i32>::new();

    route_ipv4_add(&mut top, "0.0.0.0/4", 4);
    assert_eq!(top.iter().count(), 1);

    route_ipv4_add(&mut top, "0.0.0.0/5", 5);
    assert_eq!(top.iter().count(), 2);

    route_ipv4_delete(&mut top, "0.0.0.0/4");
    assert_eq!(top.iter().count(), 1);
}
