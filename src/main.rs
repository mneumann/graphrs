mod graph;
mod index_type;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Debug;
use std::result::Result;
use graph::{MDGraph, WeightType, Unweighted};
use index_type::{IndexType, NodeIndex, EdgeIndex, DefIndex};

fn read_sgf<NodeWt, EdgeWt, NodeIx, EdgeIx, R, I>(rd: &mut R)
                                                  -> MDGraph<NodeWt, EdgeWt, NodeIx, EdgeIx>
    where NodeWt: WeightType + FromStr<Err = I>,
          EdgeWt: WeightType + FromStr<Err = I>,
          NodeIx: IndexType,
          EdgeIx: IndexType,
          I: Debug,
          R: BufRead
{
    let mut meta: Option<(bool, usize, usize)> = None;

    let mut graph = MDGraph::new();

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
                //
                // Some("u") => {
                // false
                // }
                //
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

            graph.reserve_nodes(num_nodes);
            graph.reserve_edges(num_edges);
            let _ = graph.add_nodes(num_nodes);
        } else {
            let mut i = line.splitn(2, '|');
            let (node_id, node_weight) = match i.next() {
                Some(ns) => {
                    let mut it = ns.splitn(2, ":");
                    let node_id: usize = it.next().unwrap().parse().unwrap();
                    let node_weight: Option<NodeWt> = it.next().map(|s| s.parse().unwrap());
                    (node_id, node_weight)
                }
                _ => {
                    panic!("Invalid format");
                }
            };
            if let Some(nw) = node_weight {
                *graph.get_node_weight_mut(NodeIndex::new(node_id)) = nw;
            }

            let edge_s = i.next().unwrap();
            for es in edge_s.split(',') {
                let mut it = es.splitn(2, ":");
                let target_id: usize = it.next()
                                         .unwrap()
                                         .parse()
                                         .unwrap();

                let edge_weight: Option<EdgeWt> = it.next()
                                                    .map(|s| s.parse::<EdgeWt>().unwrap());

                match edge_weight {
                    Some(ew) => {
                        let _ = graph.add_edge_with_weight(NodeIndex::new(node_id),
                                                           NodeIndex::new(target_id),
                                                           ew);
                    }
                    None => {
                        let _ = graph.add_edge(NodeIndex::new(node_id), NodeIndex::new(target_id));
                    }
                }
            }
        }
    }

    assert!(meta.unwrap().1 == graph.node_count());
    assert!(meta.unwrap().2 == graph.edge_count());
    return graph;
}


fn main() {
    let f = File::open("er_100_0_1.sgf").unwrap();
    let mut f = BufReader::new(f);
    // let graph: MDGraph<f32, f32> = read_sgf(&mut f);
    let graph: MDGraph = read_sgf(&mut f);
    println!("{:?}", graph);
}
