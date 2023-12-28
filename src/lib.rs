use std::collections::VecDeque;

use petgraph::{
    stable_graph::{IndexType, NodeIndex},
    visit::{EdgeIndexable, EdgeRef, NodeIndexable},
    EdgeType, Graph,
};

pub trait MaxFlowExt<Ix> {
    fn max_flow(&self, from: NodeIndex<Ix>, to: NodeIndex<Ix>) -> i32;
}

impl<N, Ty, Ix> MaxFlowExt<Ix> for Graph<N, i32, Ty, Ix>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    fn max_flow(&self, from: NodeIndex<Ix>, to: NodeIndex<Ix>) -> i32 {
        let s = from;
        let t = to;
        let mut flow = vec![0; self.edge_bound()];
        let mut total_flow = 0;

        loop {
            let mut pred: Vec<Option<_>> = vec![None; self.node_bound()];
            let mut queue = VecDeque::new();
            queue.push_back(s);

            while !queue.is_empty() && pred[t.index()].is_none() {
                let current = queue.pop_front().unwrap();
                for e in self.edges(current) {
                    let et = e.target().index();
                    let ei = e.id().index();
                    if pred[et].is_none() && et != s.index() && *e.weight() > flow[ei] {
                        pred[et] = Some(e);
                        queue.push_back(e.target())
                    }
                }
            }

            if pred[t.index()].is_some() {
                let mut df = i32::MAX;
                let mut n = t;
                while let Some(e) = pred[n.index()] {
                    df = df.min(e.weight() - flow[e.id().index()]);
                    n = e.source();
                }

                n = t;
                while let Some(e) = pred[n.index()] {
                    flow[e.id().index()] += df;
                    if let Some(e) = self.find_edge(e.target(), e.source()) {
                        flow[e.index()] -= df;
                    }
                    n = e.source();
                }
                total_flow += df;
            }

            if pred[t.index()].is_none() {
                return total_flow;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MaxFlowExt;
    use petgraph::graph::DiGraph;

    #[test]
    fn graph_from_wikipedia() {
        let mut gr = DiGraph::new();

        let a = gr.add_node('A');
        let b = gr.add_node('B');
        let c = gr.add_node('C');
        let d = gr.add_node('D');
        let e = gr.add_node('E');
        let f = gr.add_node('F');
        let g = gr.add_node('G');

        gr.add_edge(a, b, 3);
        gr.add_edge(a, d, 3);
        gr.add_edge(b, c, 4);
        gr.add_edge(c, a, 3);
        gr.add_edge(c, d, 1);
        gr.add_edge(c, e, 2);
        gr.add_edge(d, e, 2);
        gr.add_edge(d, f, 6);
        gr.add_edge(e, b, 1);
        gr.add_edge(e, g, 1);
        gr.add_edge(f, g, 9);

        assert_eq!(5, gr.max_flow(a, g));
    }
}
