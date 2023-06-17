// use std::collections::HashSet;
// use std::iter::FromIterator;

use crate::prelude::Graph;

#[derive(Debug)]
pub struct RuleError;

#[derive(Debug, Clone)]
pub struct Rule<T> {
    lhs: Graph<T>,
    rhs: Graph<T>,
    name: String,
    equiv: bool,
}

impl<T> Rule<T> {
    pub fn new(
        lhs: Graph<T>,
        rhs: Graph<T>,
        name: &str,
        equiv: bool,
    ) -> Result<Self, RuleError> {
        unimplemented!()
        // if lhs.inputs().len() != rhs.inputs().len()
        //     || lhs.outputs().len() != rhs.outputs().len()
        // {
        //     return Err(RuleError);
        // }
        // Ok(Self {
        //     lhs,
        //     rhs,
        //     name: String::from(name),
        //     equiv,
        // })
    }

    // pub fn copy(&self) -> Self {
    //     Self {
    //         lhs: self.lhs.copy(),
    //         rhs: self.rhs.copy(),
    //         name: self.name.clone(),
    //         equiv: self.equiv,
    //     }
    // }

    pub fn converse(&self) -> Self {
        let name = if self.name.starts_with('-') {
            self.name[1..].to_string()
        } else {
            format!("-{}", self.name)
        };

        // Self {
        //     lhs: self.rhs.copy(),
        //     rhs: self.lhs.copy(),
        //     name,
        //     equiv: true,
        // }
        unimplemented!()
    }

    pub fn is_left_linear(&self) -> bool {
        // let mut verts = HashSet::new();
        // let inputs = HashSet::from_iter(self.lhs.inputs());
        // let outputs =
        //     HashSet::from_iter(self.lhs.outputs());

        // for v in inputs.union(&outputs) {
        //     if verts.contains(v) {
        //         return false;
        //     }
        //     verts.insert(v);
        // }

        // true
        unimplemented!()
    }
}
