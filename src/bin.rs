use flow_rs::MaxFlowExt;
use petgraph::prelude::*;

fn main() {
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

    println!("{}", gr.max_flow(a, g))
}