use crate::prelude::*;

use std::collections::{
    HashMap,
    HashSet,
};

pub type Edge = usize;

fn layer_decomp<T>(g: &mut Graph<T>) -> Vec<Vec<usize>> {
    // let mut e_layers: Vec<Vec<usize>> = Vec::new();
    // let mut v_layer: Vec<usize> = Vec::new();
    // let mut v_placed: HashSet<usize> = HashSet::new();

    // // First, mark all of the inputs as 'placed' and add dummy edges for any input that is also an output
    // let outputs: HashSet<usize> = g.outputs().collect();
    // for v in g.inputs() {
    //     if outputs.contains(&v) {
    //         g.insert_id_after(v);
    //     }
    //     v_layer.push(v);
    //     v_placed.insert(v);
    // }

    // let mut new_ids: HashSet<usize> = HashSet::new();
    // let mut edges: HashSet<Edge> = g.edges().collect();

    // // Next, place edges in layers
    // while !edges.is_empty() {
    //     let mut ready: HashSet<Edge> = HashSet::new();
    //     for e in &edges {
    //         if g.source(e).all(|v| v_placed.contains(&v)) {
    //             ready.insert(*e);
    //         }
    //     }

    //     let outputs: HashSet<usize> = g.outputs().collect();
    //     for v in &v_layer {
    //         if outputs.contains(v)
    //             || g.out_edges(*v)
    //                 .any(|e| !ready.contains(&e))
    //         {
    //             let id = g.insert_id_after(*v);
    //             new_ids.insert(id);
    //             ready.insert(id);
    //         }
    //     }

    //     let mut e_layer: Vec<Edge> = Vec::new();
    //     for e in &ready {
    //         e_layer.push(*e);
    //         edges.remove(e);
    //     }

    //     if e_layer.iter().all(|e| new_ids.contains(e)) {
    //         panic!("Could not make progress. Is graph acyclic?");
    //     }

    //     e_layers.push(e_layer);

    //     if !edges.is_empty() {
    //         v_layer = Vec::new();
    //         for e in &e_layer {
    //             for v in g.target(*e) {
    //                 v_placed.insert(v);
    //                 v_layer.push(v);
    //             }
    //         }
    //     }
    // }

    // // Finally attempt to minimize crossings by sorting edges according to the ideal positions of their source and
    // // target vertices. This is done in a forward (it=0) and backward (it=1) pass.
    // for it in 0..2 {
    //     let rng = if it == 0 {
    //         0..e_layers.len()
    //     } else {
    //         (0..e_layers.len()).rev()
    //     };

    //     for j in rng {
    //         let inp: Vec<usize> = if j > 0 {
    //             e_layers[j - 1]
    //                 .iter()
    //                 .flat_map(|e| g.target(*e))
    //                 .collect()
    //         } else {
    //             g.inputs().collect()
    //         };

    //         let inp_pos: HashMap<usize, f32> = inp
    //             .iter()
    //             .enumerate()
    //             .map(|(i, v)| {
    //                 (*v, i as f32 / inp.len() as f32)
    //             })
    //             .collect();

    //         let outp_pos: Option<HashMap<usize, f32>> =
    //             if it != 0 {
    //                 let outp: Vec<usize> =
    //                     if j < e_layers.len() - 1 {
    //                         e_layers[j + 1]
    //                             .iter()
    //                             .flat_map(|e| g.source(*e))
    //                             .collect()
    //                     } else {
    //                         g.outputs().collect()
    //                     };

    //                 Some(
    //                     outp.iter()
    //                         .enumerate()
    //                         .map(|(i, v)| {
    //                             (
    //                                 *v,
    //                                 i as f32
    //                                     / outp.len() as f32,
    //                             )
    //                         })
    //                         .collect(),
    //                 )
    //             } else {
    //                 None
    //             };

    //         let mut e_pos: HashMap<Edge, f32> =
    //             HashMap::new();
    //         for e in &e_layers[j] {
    //             let src = g.source(*e);
    //             let sum_pos = src
    //                 .iter()
    //                 .map(|v| inp_pos[v])
    //                 .sum::<f32>();
    //             let src_len = src.len() as f32;
    //             e_pos.insert(*e, sum_pos / src_len);

    //             if let Some(outp_pos) = &outp_pos {
    //                 let tgt = g.target(*e);
    //                 let sum_pos = tgt
    //                     .iter()
    //                     .map(|v| outp_pos[v])
    //                     .sum::<f32>();
    //                 let tgt_len = tgt.len() as f32;
    //                 e_pos.entry(*e).and_modify(|e_pos| {
    //                     *e_pos += 2.0 * sum_pos / tgt_len
    //                 });
    //             }
    //         }

    //         e_layers[j].sort_by(|a, b| {
    //             e_pos[a].partial_cmp(&e_pos[b]).unwrap()
    //         });
    //     }
    // }

    // e_layers
    unimplemented!()
}

fn perm_to_s(perm: &[usize]) -> String {
    match perm.len() {
        1 => "id".to_string(),
        2 => "sw".to_string(),
        _ => format!(
            "sw{}",
            perm.iter()
                .map(|&x| x.to_string())
                .collect::<String>()
        ),
    }
}

fn split_perm(perm: &[usize]) -> Vec<Vec<usize>> {
    let mut perms: Vec<Vec<usize>> = Vec::new();
    let mut rest: Vec<usize> = perm.to_vec();

    while !rest.is_empty() {
        let mut m = 0;
        for (i, &x) in rest.iter().enumerate() {
            m = m.max(x);
            if m <= i {
                perms.push(rest[..i + 1].to_vec());
                rest = rest[i + 1..]
                    .iter()
                    .map(|&y| y - (i + 1))
                    .collect();
                break;
            }
        }
    }

    perms
}

fn graph_to_term<T>(g: &mut Graph<T>) -> String {
    unimplemented!()
    // let mut g = g.clone();
    // let e_layers = layer_decomp(&mut g);

    // let mut in_layer: Vec<usize> = g.inputs().collect();
    // let mut seq: Vec<String> = Vec::new();

    // for i in 0..e_layers.len() {
    //     let v_pos: HashMap<usize, usize> = in_layer
    //         .iter()
    //         .enumerate()
    //         .map(|(j, &v)| (v, j))
    //         .collect();

    //     let out_layer: Vec<usize> = e_layers[i]
    //         .iter()
    //         .flat_map(|&e| g.source(e))
    //         .collect();
    //     let v_perm: Vec<usize> =
    //         out_layer.iter().map(|&v| v_pos[&v]).collect();

    //     if v_perm
    //         != (0..v_perm.len()).collect::<Vec<usize>>()
    //     {
    //         let perms = split_perm(&v_perm);
    //         let perm_str = perms
    //             .iter()
    //             .map(|p| perm_to_s(p))
    //             .collect::<Vec<String>>()
    //             .join(" * ");
    //         seq.push(perm_str);
    //     }

    //     let par: Vec<String> = e_layers[i]
    //         .iter()
    //         .map(|&e| g.edge_data(e).value.to_string())
    //         .collect();
    //     seq.push(par.join(" * "));

    //     in_layer = e_layers[i]
    //         .iter()
    //         .flat_map(|&e| g.target(e))
    //         .collect();
    // }

    // let v_pos: HashMap<usize, usize> = in_layer
    //     .iter()
    //     .enumerate()
    //     .map(|(j, &v)| (v, j))
    //     .collect();

    // let out_layer: Vec<usize> = g.outputs().collect();
    // let v_perm: Vec<usize> =
    //     out_layer.iter().map(|&v| v_pos[&v]).collect();

    // if v_perm != (0..v_perm.len()).collect::<Vec<usize>>() {
    //     let perms = split_perm(&v_perm);
    //     let perm_str = perms
    //         .iter()
    //         .map(|p| perm_to_s(p))
    //         .collect::<Vec<String>>()
    //         .join(" * ");
    //     seq.push(perm_str);
    // }

    // seq.join(" ; ")
}
