use index_type::{IndexType, NodeIndex, EdgeIndex, DefIndex};
use std::str::FromStr;
use std::default::Default;
use std::fmt;

pub trait WeightType: Sized + fmt::Debug + Default + Copy {}

pub struct Unweighted;

/// A weighted, multi-edged, directed graph. Edges are represented by a unique index EdgeIx. Nodes by their NodeIx.
pub struct MDGraph<NodeWt: WeightType = Unweighted,
                   EdgeWt: WeightType = Unweighted,
                   NodeIx: IndexType = DefIndex,
                   EdgeIx: IndexType = DefIndex>
{
    node_weights: Vec<NodeWt>,
    edge_weights: Vec<EdgeWt>,

    // in_edges[v] contains all nodes u with (u, v) in E.
    in_edges: Vec<Vec<(NodeIndex<NodeIx>, EdgeIndex<EdgeIx>)>>,

    // out_edges[u] contains all nodes v with (u, v) in E.
    out_edges: Vec<Vec<(NodeIndex<NodeIx>, EdgeIndex<EdgeIx>)>>,
}

impl<NodeWt: WeightType, EdgeWt: WeightType, NodeIx: IndexType, EdgeIx: IndexType> MDGraph<NodeWt, EdgeWt, NodeIx, EdgeIx> {
    pub fn new() -> MDGraph<NodeWt, EdgeWt, NodeIx, EdgeIx> {
        MDGraph {
            node_weights: Vec::new(),
            edge_weights: Vec::new(),
            in_edges: Vec::new(),
            out_edges: Vec::new(),
        }
    }

    pub fn reserve_nodes(&mut self, additional: usize) {
        self.node_weights.reserve(additional);
        self.in_edges.reserve(additional);
        self.out_edges.reserve(additional);
    }

    pub fn reserve_edges(&mut self, additional: usize) {
        self.edge_weights.reserve(additional);
    }

    pub fn add_node_with_weight(&mut self, weight: NodeWt) -> NodeIndex<NodeIx> {
        let idx = NodeIndex::new(self.node_weights.len());
        self.node_weights.push(weight);
        self.in_edges.push(Vec::new());
        self.out_edges.push(Vec::new());
        idx
    }

    pub fn add_node(&mut self) -> NodeIndex<NodeIx> {
        self.add_node_with_weight(NodeWt::default())
    }

    /// Returns first and last index.
    pub fn add_nodes(&mut self, n: usize) -> (NodeIndex<NodeIx>, NodeIndex<NodeIx>) {
        assert!(n > 0);
        let first_idx = self.add_node();
        let mut last_idx = first_idx;
        for _ in 1..n {
            last_idx = self.add_node();
        }
        return (first_idx, last_idx); // NOTE: Inclusive index!
    }

    pub fn add_edge_with_weight(&mut self,
                                source: NodeIndex<NodeIx>,
                                target: NodeIndex<NodeIx>,
                                weight: EdgeWt)
                                -> EdgeIndex<EdgeIx> {
        assert!(source.index() < self.node_weights.len());
        assert!(target.index() < self.node_weights.len());

        let idx = EdgeIndex::new(self.edge_weights.len());
        self.edge_weights.push(weight);

        self.out_edges[source.index()].push((target, idx));
        self.in_edges[target.index()].push((source, idx));

        idx
    }

    pub fn add_edge(&mut self,
                    source: NodeIndex<NodeIx>,
                    target: NodeIndex<NodeIx>)
                    -> EdgeIndex<EdgeIx> {
        self.add_edge_with_weight(source, target, EdgeWt::default())
    }

    pub fn in_degree(&self, node: NodeIndex<NodeIx>) -> usize {
        self.in_edges[node.index()].len()
    }

    pub fn out_degree(&self, node: NodeIndex<NodeIx>) -> usize {
        self.out_edges[node.index()].len()
    }

    pub fn get_node_weight(&self, node: NodeIndex<NodeIx>) -> &NodeWt {
        &self.node_weights[node.index()]
    }

    pub fn get_node_weight_mut(&mut self, node: NodeIndex<NodeIx>) -> &mut NodeWt {
        &mut self.node_weights[node.index()]
    }

    pub fn get_edge_weight(&self, edge: EdgeIndex<EdgeIx>) -> &EdgeWt {
        &self.edge_weights[edge.index()]
    }

    pub fn get_edge_weight_mut(&mut self, edge: EdgeIndex<EdgeIx>) -> &mut EdgeWt {
        &mut self.edge_weights[edge.index()]
    }

    pub fn in_edges_of(&self,
                       node: NodeIndex<NodeIx>)
                       -> &[(NodeIndex<NodeIx>, EdgeIndex<EdgeIx>)] {
        &self.in_edges[node.index()]
    }

    pub fn out_edges_of(&self,
                        node: NodeIndex<NodeIx>)
                        -> &[(NodeIndex<NodeIx>, EdgeIndex<EdgeIx>)] {
        &self.out_edges[node.index()]
    }
}
