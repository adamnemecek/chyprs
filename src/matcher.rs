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

#[derive(Clone)]
pub struct Match<'a, T> {
    dom: Option<&'a Graph<T>>,
    cod: Option<&'a Graph<T>>,
    vmap: HashMap<usize, usize>,
    vimg: HashSet<usize>,
    emap: HashMap<usize, usize>,
    eimg: HashSet<usize>,
}

// impl<'a> Match<'a> {
//     pub fn new(dom: Option<&'a Graph>, cod: Option<&'a Graph>, m: Option<&Match<'a>>) -> Result<Self, &'static str> {
//         if let Some(existing_match) = m {
//             Ok(Match {
//                 dom: existing_match.dom,
//                 cod: existing_match.cod,
//                 vmap: existing_match.vmap.clone(),
//                 vimg: existing_match.vimg.clone(),
//                 emap: existing_match.emap.clone(),
//                 eimg: existing_match.eimg.clone(),
//             })
//         } else if let (Some(d), Some(c)) = (dom, cod) {
//             Ok(Match {
//                 dom: Some(d),
//                 cod: Some(c),
//                 vmap: HashMap::new(),
//                 vimg: HashSet::new(),
//                 emap: HashMap::new(),
//                 eimg: HashSet::new(),
//             })
//         } else {
//             Err("Must provide either a match or a pair of graphs")
//         }
//     }

//     pub fn copy(&self) -> Self {
//         Match {
//             dom: self.dom,
//             cod: self.cod,
//             vmap: self.vmap.clone(),
//             vimg: self.vimg.clone(),
//             emap: self.emap.clone(),
//             eimg: self.eimg.clone(),
//         }
//     }

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
