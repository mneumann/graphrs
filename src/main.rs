use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;
use std::result::Result;

// optional edge-weight + target node
type EdgeLink<EW> = (Option<EW>, usize);
type Node<NW, EW> = (Option<NW>, Vec<EdgeLink<EW>>);

fn read_sgf<I, R, NW, EW>(rd: &mut R) -> (Vec<Node<NW, EW>>, bool)
    where I: Debug,
          R: BufRead,
          NW: FromStr<Err = I>,
          EW: FromStr<Err = I>
{
    let mut meta: Option<(bool, usize, usize)> = None;
    let mut nodes: Vec<Node<NW, EW>> = Vec::new();
    let mut cnt_edges: usize = 0;

    for line in rd.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with("#") {
            // skip comment
            continue;
        }

        if meta.is_none() {
            // First line is meta
            let mut m = line.split_whitespace();
            let directed = match m.next() {
                Some("d") => {
                    true
                }
                Some("u") => {
                    false
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            let num_nodes: usize = match m.next() {
                Some(ns) => {
                    ns.parse().unwrap()
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            let num_edges: usize = match m.next() {
                Some(ns) => {
                    ns.parse().unwrap()
                }
                _ => {
                    panic!("Invalid format");
                }
            };

            meta = Some((directed, num_nodes, num_edges));
        } else {
            let mut i = line.splitn(2, '|');
            let (node_id, node_weight) = match i.next() {
                Some(ns) => {
                    let mut it = ns.splitn(2, ":");
                    let node_id: usize = it.next().unwrap().parse().unwrap();
                    let node_weight: Option<NW> = it.next().map(|s| s.parse().unwrap());
                    (node_id, node_weight)
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            let edge_s = i.next().unwrap();
            let edges: Vec<EdgeLink<EW>> = edge_s.split(',')
                                                 .map(|es| {
                                                     let mut it = es.splitn(2, ":");
                                                     let edge_id: usize = it.next()
                                                                            .unwrap()
                                                                            .parse()
                                                                            .unwrap();
                                                     let edge_weight: Option<EW> =
                                                         it.next()
                                                           .map(|s| s.parse::<EW>().unwrap());
                                                     (edge_weight, edge_id)
                                                 })
                                                 .collect();
            assert!(node_id == nodes.len());
            cnt_edges += edges.len();
            nodes.push((node_weight, edges)); // parse node weight
        }
    }

    assert!(meta.unwrap().1 == nodes.len());
    assert!(meta.unwrap().2 == cnt_edges);
    return (nodes, meta.unwrap().0);
}


fn main() {
    let f = File::open("er_100_0_1.sgf").unwrap();
    let mut f = BufReader::new(f);
    let (nodes, directed): (Vec<Node<f32, f32>>, bool) = read_sgf(&mut f);
    println!("{:?}", nodes);
}
