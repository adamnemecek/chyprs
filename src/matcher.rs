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
        //     .unwrap()
        //     .in_edges(v)
        //     .all(|e| self.emap.contains_key(&e))
        //     && self
        //         .dom
        //         .unwrap()
        //         .out_edges(v)
        //         .all(|e| self.emap.contains_key(&e))
        unimplemented!()
    }

    pub fn map_scalars(&mut self) -> bool {
        // let mut cod_sc: Vec<(usize, T)> = Vec::new();

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
}
