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

    //     pub fn try_add_vertex(&mut self, v: usize, cod_v: usize) -> bool {
    //         match_log(&format!("trying to add vertex {} -> {} to match:", v, cod_v));
    //         match_log(&format!("{:?}", self));

    //         if self.vmap.contains_key(&v) {
    //             match_log(&format!("vertex already mapped to {}", self.vmap[&v]));
    //             return self.vmap[&v] == cod_v;
    //         }

    //         let v_val = self.dom.unwrap().vertex_data(v).value;
    //         let cod_v_val = self.cod.unwrap().vertex_data(cod_v).value;

    //         if v_val != cod_v_val {
    //             match_log(&format!("vertex failed: values {} != {}", v_val, cod_v_val));
    //             return false;
    //         }

    //         if self.cod.unwrap().is_boundary(cod_v) && !self.dom.unwrap().is_boundary(v) {
    //             match_log("vertex failed: cod v is boundary but dom v is not");
    //             return false;
    //         }

    //         if self.vimg.contains(&cod_v) {
    //             if !self.dom.unwrap().is_boundary(v) {
    //                 match_log("vertex failed: non-injective on interior vertex");
    //                 return false;
    //             }
    //             for (dv, cv) in self.vmap.iter() {
    //                 if cv == &cod_v && !self.dom.unwrap().is_boundary(*dv) {
    //                     match_log("vertex failed: non-injective on interior vertex");
    //                     return false;
    //                 }
    //             }
    //         }

    //         self.vmap.insert(v, cod_v);
    //         self.vimg.insert(cod_v);

    //         if !self.dom.unwrap().is_boundary(v) {
    //             if self.dom.unwrap().in_edges(v.
}
