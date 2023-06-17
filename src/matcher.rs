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

    fn try_add_vertex(
        &mut self,
        v: usize,
        cod_v: usize,
    ) -> bool {
        // match_log(&format!("trying to add vertex {} -> {} to match:", v, cod_v));
        // match_log(&format!("{:?}", self));

        // if self.vmap.contains_key(&v) {
        //     match_log(&format!("vertex already mapped to {}", self.vmap[&v]));
        //     return self.vmap[&v] == cod_v;
        // }

        // let v_val = self.dom.unwrap().vertex_data(v).value;
        // let cod_v_val = self.cod.unwrap().vertex_data(cod_v).value;

        // if v_val != cod_v_val {
        //     match_log(&format!("vertex failed: values {} != {}", v_val, cod_v_val));
        //     return false;
        // }

        // if self.cod.unwrap().is_boundary(cod_v) && !self.dom.unwrap().is_boundary(v) {
        //     match_log("vertex failed: cod v is boundary but dom v is not");
        //     return false;
        // }

        // if self.vimg.contains(&cod_v) {
        //     if !self.dom.unwrap().is_boundary(v) {
        //         match_log("vertex failed: non-injective on interior vertex");
        //         return false;
        //     }
        //     for (dv, cv) in self.vmap.iter() {
        //         if cv == &cod_v && !self.dom.unwrap().is_boundary(*dv) {
        //             match_log("vertex failed: non-injective on interior vertex");
        //             return false;
        //         }
        //     }
        // }

        // self.vmap.insert(v, cod_v);
        // self.vimg.insert(cod_v);

        // if !self.dom.unwrap().is_boundary(v) {
        //     if self.dom.unwrap().in_edges(v).count() != self.cod.unwrap().in_edges(cod_v).count() {
        //         match_log("vertex failed: in_edges cannot satisfy gluing conds");
        //         return false;
        //     }
        //     if self.dom.unwrap().out_edges(v).count() != self.cod.unwrap().out_edges(cod_v).count() {
        //         match_log("vertex failed: out_edges cannot satisfy gluing conds");
        //         return false;
        //     }
        // }

        // match_log("vertex success");
        // true
        unimplemented!()
    }

    fn try_add_edge(
        &mut self,
        e: usize,
        cod_e: usize,
    ) -> bool {
        // match_log(&format!(
        //     "trying to add edge {} -> {} to match:",
        //     e, cod_e
        // ));
        // match_log(&format!("{:?}", self));

        // let e_val = self.dom.unwrap().edge_data(e).value;
        // let cod_e_val =
        //     self.cod.unwrap().edge_data(cod_e).value;

        // if e_val != cod_e_val {
        //     match_log(&format!(
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

        // let s = self.dom.unwrap().source(e);
        // let cod_s = self.cod.unwrap().source(cod_e);
        // let t = self.dom.unwrap().target(e);
        // let cod_t = self.cod.unwrap().target(cod_e);

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

    fn dom_nhd_mapped(&self, v: usize) -> bool {
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
}
