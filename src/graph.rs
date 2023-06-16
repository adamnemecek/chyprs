use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

#[derive(Debug)]
struct GraphError;

#[derive(Clone, Debug)]
struct VData<T> {
    value: T,
    x: f32,
    y: f32,
    highlight: bool,
    in_edges: HashSet<usize>,
    out_edges: HashSet<usize>,
    in_indices: HashSet<usize>,
    out_indices: HashSet<usize>,
}

impl<T> VData<T> {
    fn new(x: f32, y: f32, value: T) -> Self {
        Self {
            value,
            x,
            y,
            highlight: false,
            in_edges: HashSet::new(),
            out_edges: HashSet::new(),
            in_indices: HashSet::new(),
            out_indices: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct EData<T> {
    value: T,
    highlight: bool,
    x: f32,
    y: f32,
    s: Vec<usize>,
    t: Vec<usize>,
    fg: String,
    bg: String,
    hyper: bool,
}

impl<T> EData<T> {
    fn new(
        s: Option<Vec<usize>>,
        t: Option<Vec<usize>>,
        value: T,
        x: f32,
        y: f32,
        fg: String,
        bg: String,
        hyper: bool,
    ) -> Self {
        Self {
            value,
            highlight: false,
            x,
            y,
            s: s.unwrap_or_else(|| vec![]),
            t: t.unwrap_or_else(|| vec![]),
            fg,
            bg,
            hyper,
        }
    }

    fn box_size(&self) -> usize {
        if self.s.len() <= 1 && self.t.len() <= 1 {
            1
        } else {
            2
        }
    }
}

#[derive(Debug)]
struct Graph<T> {
    vdata: HashMap<usize, VData<T>>,
    edata: HashMap<usize, EData<T>>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    vindex: usize,
    eindex: usize,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self {
            vdata: HashMap::new(),
            edata: HashMap::new(),
            inputs: vec![],
            outputs: vec![],
            vindex: 0,
            eindex: 0,
        }
    }

    // fn copy(&self) -> Self {
    //     Self {
    //         vdata: self.vdata.clone(),
    //         edata: self.edata.clone(),
    //         inputs: self.inputs.clone(),
    //         outputs: self.outputs.clone(),
    //         vindex: self.vindex,
    //         eindex: self.eindex,
    //     }
    // }

    fn vertices(&self) -> impl Iterator<Item = &usize> {
        self.vdata.keys()
    }

    fn edges(&self) -> impl Iterator<Item = &usize> {
        self.edata.keys()
    }

    fn num_vertices(&self) -> usize {
        self.vdata.len()
    }

    fn num_edges(&self) -> usize {
        self.edata.len()
    }

    fn vertex_data(&self, v: usize) -> Option<&VData<T>> {
        self.vdata.get(&v)
    }

    fn edge_data(&self, e: usize) -> Option<&EData<T>> {
        self.edata.get(&e)
    }

    fn in_edges(&self, v: usize) -> Option<&HashSet<usize>> {
        self.vdata.get(&v).map(|data| &data.in_edges)
    }

    // fn out_edges(&self, v: usize) -> Option<&.
}
