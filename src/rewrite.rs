// use crate::matcher::Match;
// use crate::prelude::{
//     Edge,
//     Graph,
//     // Vertex,
// };
// use crate::rule::Rule;
// use std::collections::HashMap;
// use std::iter::FromIterator;
// use std::vec::Vec;

// fn dpo<T>(r: &Rule<T>, m: &Match<T>) -> Vec<Match<T>> {
//     let mut in_map: HashMap<usize, usize> = HashMap::new();
//     let mut out_map: HashMap<usize, usize> = HashMap::new();

//     let mut ctx = m.cod.clone();
//     for e in r.lhs.edges() {
//         ctx.remove_edge(&m.emap[&e]);
//     }
//     for v in r.lhs.vertices() {
//         let v1 = m.vmap[&v];
//         if r.lhs.is_boundary(v) {
//             let in_c =
//                 r.lhs.vertex_data(&v).in_indices.len();
//             let out_c =
//                 r.lhs.vertex_data(&v).out_indices.len();
//             if in_c == 1 && out_c == 1 {
//                 let (v1i, v1o) = ctx.explode_vertex(v1);
//                 if v1i.len() == 1 && v1o.len() == 1 {
//                     in_map.insert(v, v1i[0]);
//                     out_map.insert(v, v1o[0]);
//                 } else {
//                     panic!("Rewriting modulo Frobenius not yet supported.");
//                 }
//             } else if in_c > 1 || out_c > 1 {
//                 panic!("Rewriting modulo Frobenius not yet supported.");
//             }
//         } else {
//             ctx.remove_vertex(v1);
//         }
//     }

//     let mut h = ctx;
//     let mut m1 = Match::new(&r.rhs, &h);

//     for (vl, vr) in
//         r.lhs.inputs().iter().zip(r.rhs.inputs())
//     {
//         m1.vmap.insert(
//             vr,
//             in_map.get(vl).copied().unwrap_or(m.vmap[vl]),
//         );
//     }

//     for (vl, vr) in
//         r.lhs.outputs().iter().zip(r.rhs.outputs())
//     {
//         let vr1 = *out_map.get(vl).unwrap_or(&m.vmap[vl]);
//         if let Some(&vr_mapped) = m1.vmap.get(&vr) {
//             h.merge_vertices(vr_mapped, vr1);
//         } else {
//             m1.vmap.insert(vr, vr1);
//         }
//     }

//     for v in r.rhs.vertices() {
//         if !r.rhs.is_boundary(v) {
//             let vd = r.rhs.vertex_data(v);
//             let v1 = h.add_vertex(vd.x, vd.y, vd.value);
//             m1.vmap.insert(v, v1);
//             m1.vimg.insert(v1);
//         }
//     }

//     for e in r.rhs.edges() {
//         let ed = r.rhs.edge_data(e);
//         let s =
//             ed.s.iter()
//                 .map(|v| m1.vmap[v])
//                 .collect::<Vec<_>>();
//         let t =
//             ed.t.iter()
//                 .map(|v| m1.vmap[v])
//                 .collect::<Vec<_>>();
//         let e1 = h.add_edge(
//             &s, &t, ed.value, ed.x, ed.y, ed.fg, ed.bg,
//             ed.hyper,
//         );
//         m1.emap.insert(e, e1);
//         m1.eimg.insert(e1);
//     }

//     vec![m1]
// }

// fn rewrite(r: &Rule, m: &Match) -> Graph {
//     match dpo(r, m).into_iter().next() {
//         Some(result) => result.cod.clone(),
//         None => panic!("Rewrite has no valid context"),
//     }
// }
