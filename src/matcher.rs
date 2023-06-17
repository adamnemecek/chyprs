use std::collections::{
    HashMap,
    HashSet,
};
// use std::fmt;
// use std::iter::Iterator;
// use crate::graph::Graph;
// use crate::rule::Rule;
pub use crate::prelude::*;

// const DEBUG_MATCH: bool = false;

// fn match_log(s: &str) {
//     if DEBUG_MATCH {
//         println!("{}", s);
//     }
// }

#[derive(Clone, Debug)]
pub struct Match<'a, T> {
    dom: &'a Graph<T>,
    cod: &'a Graph<T>,
    vmap: HashMap<usize, usize>,
    vimg: HashSet<usize>,
    emap: HashMap<usize, usize>,
    eimg: HashSet<usize>,
}

impl<'a, T> Match<'a, T> {
    pub fn new(
        dom: &'a Graph<T>,
        cod: &'a Graph<T>,
    ) -> Self {
        Self {
            dom,
            cod,
            vmap: <_>::default(),
            vimg: <_>::default(),
            emap: <_>::default(),
            eimg: <_>::default(),
        }
    }

    pub fn try_add_vertex(
        &mut self,
        v: usize,
        cod_v: usize,
    ) -> bool {
        // println!(!("trying to add vertex {} -> {} to match:", v, cod_v));
        // println!(!("{:?}", self));

        if self.vmap.contains_key(&v) {
            println!(
                "vertex already mapped to {}",
                self.vmap[&v]
            );
            return self.vmap[&v] == cod_v;
        }

        // let v_val = self.dom.vertex_data(v).value;
        // let cod_v_val = self.cod.vertex_data(cod_v).value;

        // if v_val != cod_v_val {
        //     println!(!("vertex failed: values {} != {}", v_val, cod_v_val));
        //     return false;
        // }

        // if self.cod.is_boundary(cod_v) && !self.dom.is_boundary(v) {
        //     match_log("vertex failed: cod v is boundary but dom v is not");
        //     return false;
        // }

        // if self.vimg.contains(&cod_v) {
        //     if !self.dom.is_boundary(v) {
        //         match_log("vertex failed: non-injective on interior vertex");
        //         return false;
        //     }
        //     for (dv, cv) in self.vmap.iter() {
        //         if cv == &cod_v && !self.dom.is_boundary(*dv) {
        //             match_log("vertex failed: non-injective on interior vertex");
        //             return false;
        //         }
        //     }
        // }

        // self.vmap.insert(v, cod_v);
        // self.vimg.insert(cod_v);

        // if !self.dom.is_boundary(v) {
        //     if self.dom.in_edges(v).count() != self.cod.in_edges(cod_v).count() {
        //         match_log("vertex failed: in_edges cannot satisfy gluing conds");
        //         return false;
        //     }
        //     if self.dom.out_edges(v).count() != self.cod.out_edges(cod_v).count() {
        //         match_log("vertex failed: out_edges cannot satisfy gluing conds");
        //         return false;
        //     }
        // }

        // match_log("vertex success");
        // true
        unimplemented!()
    }

    pub fn try_add_edge(
        &mut self,
        e: usize,
        cod_e: usize,
    ) -> bool {
        // println!(!(
        //     "trying to add edge {} -> {} to match:",
        //     e, cod_e
        // ));
        // println!(!("{:?}", self));

        // let e_val = self.dom.edge_data(e).value;
        // let cod_e_val =
        //     self.cod.edge_data(cod_e).value;

        // if e_val != cod_e_val {
        //     println!(!(
        //         "edge failed: values {} != {}",
        //         e_val, cod_e_val
        //     ));
        //     return false;
        // }

        // if self.eimg.contains(&cod_e) {
        //     match_log("edge failed: non-injective");
        //     return false;
        // }

        // self.emap.insert(e, cod_e);
        // self.eimg.insert(cod_e);

        // let s = self.dom.source(e);
        // let cod_s = self.cod.source(cod_e);
        // let t = self.dom.target(e);
        // let cod_t = self.cod.target(cod_e);

        // if s.len() != cod_s.len() || t.len() != cod_t.len()
        // {
        //     match_log("edge failed: source or target len doesn't match image");
        //     return false;
        // }

        // for (v1, cod_v1) in s
        //     .iter()
        //     .chain(t.iter())
        //     .zip(cod_s.iter().chain(cod_t.iter()))
        // {
        //     if self.vmap.contains_key(v1) {
        //         if self.vmap[v1] != *cod_v1 {
        //             match_log("edge failed: inconsistent with previously mapped vertex");
        //             return false;
        //         }
        //     } else {
        //         if !self.try_add_vertex(*v1, *cod_v1) {
        //             match_log("edge failed: couldn't add a vertex");
        //             return false;
        //         }
        //     }
        // }

        // match_log("edge success");
        // true
        unimplemented!()
    }

    pub fn dom_nhd_mapped(&self, v: usize) -> bool {
        // self.dom
        //     .in_edges(v)
        //     .all(|e| self.emap.contains_key(&e))
        //     && self
        //         .dom
        //         .out_edges(v)
        //         .all(|e| self.emap.contains_key(&e))
        unimplemented!()
    }

    pub fn map_scalars(&mut self) -> bool {
        // let mut cod_sc = vec![];

        // for e in self.cod.edges() {
        //     let ed = self.cod.edge_data(e);
        //     if ed.s.is_empty() && ed.t.is_empty() {
        //         cod_sc.push((e, ed.value));
        //     }
        // }

        // for e in self.dom.edges() {
        //     println!(!("trying to map scalar edge {}", e));
        //     let ed = self.dom.edge_data(e);
        //     if !ed.s.is_empty() || !ed.t.is_empty() {
        //         continue;
        //     }

        //     let mut found = false;
        //     for i in 0..cod_sc.len() {
        //         let (e1, val) = cod_sc[i];
        //         if val == ed.value {
        //             cod_sc.remove(i);
        //             self.emap.insert(e, e1);
        //             self.eimg.insert(e1);
        //             found = true;
        //             println!(!("successfully mapped scalar {} -> {}", e, e1));
        //             break;
        //         }
        //     }

        //     if !found {
        //         println!(!("match failed: could not map scalar edge {}", e));
        //         return false;
        //     }
        // }

        // true
        unimplemented!()
    }

    pub fn more(&self) -> Vec<Self> {
        unimplemented!()
        // let mut ms: Vec<Match> = Vec::new();

        // for v in self.vmap.keys() {
        //     if self.dom_nhd_mapped(*v) {
        //         continue;
        //     }

        //     let cod_v = self.vmap[v];

        //     for e in self.dom.in_edges(*v) {
        //         if self.emap.contains_key(&e) {
        //             continue;
        //         }

        //         for cod_e in self.cod.in_edges(cod_v) {
        //             let mut m1 = self.copy();
        //             if m1.try_add_edge(e, cod_e) {
        //                 ms.push(m1);
        //             }
        //         }
        //         return ms;
        //     }

        //     for e in self.dom.out_edges(*v) {
        //         if self.emap.contains_key(&e) {
        //             continue;
        //         }

        //         for cod_e in self.cod.out_edges(cod_v) {
        //             let mut m1 = self.copy();
        //             if m1.try_add_edge(e, cod_e) {
        //                 ms.push(m1);
        //             }
        //         }
        //         return ms;
        //     }
        // }

        // for v in self.dom.vertices() {
        //     if self.vmap.contains_key(&v) {
        //         continue;
        //     }

        //     for cod_v in self.cod.vertices() {
        //         let mut m1 = self.copy();
        //         if m1.try_add_vertex(v, cod_v) {
        //             ms.push(m1);
        //         }
        //     }
        //     return ms;
        // }

        // vec![]
    }

    pub fn is_total(&self) -> bool {
        self.vmap.len() == self.dom.num_vertices()
            && self.emap.len() == self.dom.num_edges()
    }

    pub fn is_surjective(&self) -> bool {
        self.vimg.len() == self.cod.num_vertices()
            && self.eimg.len() == self.cod.num_edges()
    }

    pub fn is_injective(&self) -> bool {
        self.vmap.len() == self.vimg.len()
    }

    pub fn is_convex(&self) -> bool {
        unimplemented!()
        // if !self.is_injective() {
        //     return false;
        // }

        // let future: HashSet<_> = self
        //     .cod
        //     .successors(
        //         self.dom
        //             .outputs()
        //             .iter()
        //             .filter(|v| self.vmap.contains_key(v))
        //             .map(|v| self.vmap[v]),
        //     )
        //     .collect();

        // for v in self.dom.inputs() {
        //     if let Some(cod_v) = self.vmap.get(&v) {
        //         if future.contains(cod_v) {
        //             return false;
        //         }
        //     }
        // }

        // true
    }
}

use std::collections::VecDeque;

pub struct Matches<'a, T> {
    dom: &'a Graph<T>,
    cod: &'a Graph<T>,
    convex: bool,
    match_stack: VecDeque<Match<'a, T>>,
}

// impl<'a> Iterator for Matches<'a> {
//     type Item = Match<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(m) = self.match_stack.pop_back() {
//             if m.is_total() {
//                 match_log(format!(
//                     "got successful match:\n{}",
//                     m
//                 ));
//                 if self.convex {
//                     if m.is_convex() {
//                         match_log(
//                             "match is convex, returning",
//                         );
//                         return Some(m);
//                     } else {
//                         match_log(
//                             "match is not convex, dropping",
//                         );
//                     }
//                 } else {
//                     return Some(m);
//                 }
//             } else {
//                 self.match_stack.extend(m.more());
//             }
//         }
//         None
//     }
// }

// impl<'a> Matches<'a> {
//     fn new(dom: &'a Graph, cod: &'a Graph, initial_match: Option<Match<'a>>, convex: bool) -> Self {
//         let initial_match = initial_match.unwrap_or_else(|| Match::new(dom, cod));
//         let match_stack = if initial_match.map_scalars() {
//             vec![initial_match].into()
//         } else {
//             VecDeque::new()
//         };

//         Matches {
//             dom,
//             cod,
//             convex,
//             match_stack,
//         }
//     }
// }

// fn match_graph(dom: &Graph, cod: &Graph, convex: bool) -> Matches {
//     Matches::new(dom, cod, None, convex)
// }

// fn match_rule(r: &Rule, g: &Graph, convex: bool) -> Matches {
//     Matches::new(&r.lhs, g, None, convex)
// }

// fn find_iso(g: &Graph, h: &Graph) -> Option<Match> {
//     let g_in = g.inputs();
//     let g_out = g.outputs();
//     let h_in = h.inputs();
//     let h_out = h.outputs();
//     if g_in.len() != h_in.len() || g_out.len() != h_out.len() {
//         return None;
//     }

//     let mut m0 = Match::new(g, h);
//     for i in 0..g_in.len() {
//         if !m0.try_add_vertex(g_in[i], h_in[i]) {
//             return None;
//         }
//     }
//     for i in 0..g_out.len() {
//         if !m0.try_add_vertex(g_out[i], h_out[i]) {
//             return None;
//         }
//     }

//     for m in Matches::new(g, h, Some(m0), false) {
//         if m.is_surjective() {
//             return Some(m);
//         }
//     }

//     None
// }
