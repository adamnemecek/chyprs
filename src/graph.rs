use std::collections::{
    HashMap,
    HashSet,
};
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
pub struct Graph<T> {
    vdata: HashMap<usize, VData<T>>,
    edata: HashMap<usize, EData<T>>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    vindex: usize,
    eindex: usize,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
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

    pub fn vertices(&self) -> impl Iterator<Item = &usize> {
        self.vdata.keys()
    }

    pub fn edges(&self) -> impl Iterator<Item = &usize> {
        self.edata.keys()
    }

    pub fn num_vertices(&self) -> usize {
        self.vdata.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edata.len()
    }

    pub fn vertex_data(
        &self,
        v: usize,
    ) -> Option<&VData<T>> {
        self.vdata.get(&v)
    }

    pub fn edge_data(&self, e: usize) -> Option<&EData<T>> {
        self.edata.get(&e)
    }

    pub fn in_edges(
        &self,
        v: usize,
    ) -> Option<&HashSet<usize>> {
        self.vdata.get(&v).map(|data| &data.in_edges)
    }

    pub fn out_edges(
        &self,
        v: usize,
    ) -> Option<&HashSet<usize>> {
        self.vdata.get(&v).map(|data| &data.out_edges)
    }

    pub fn in_indices(
        &self,
        v: usize,
    ) -> Option<&HashSet<usize>> {
        self.vdata.get(&v).map(|data| &data.in_indices)
    }

    pub fn out_indices(
        &self,
        v: usize,
    ) -> Option<&HashSet<usize>> {
        self.vdata.get(&v).map(|data| &data.out_indices)
    }

    pub fn set_vertex_data(
        &mut self,
        v: usize,
        data: VData<T>,
    ) {
        self.vdata.insert(v, data);
    }

    fn set_edge_data(&mut self, e: usize, data: EData<T>) {
        self.edata.insert(e, data);
    }

    pub fn add_vertex(
        &mut self,
        x: f32,
        y: f32,
        value: T,
    ) -> usize {
        let index = self.vindex;
        self.vdata.insert(index, VData::new(x, y, value));
        self.vindex += 1;
        index
    }

    // fn add_edge(
    //     &mut self,
    //     s: Option<Vec<usize>>,
    //     t: Option<Vec<usize>>,
    //     value: T,
    //     x: f32,
    //     y: f32,
    //     fg: String,
    //     bg: String,
    //     hyper: bool,
    // ) -> usize {
    //     let index = self.eindex;
    //     self.edata
    //         .insert(index, EData::new(s, t, value, x, y, fg, bg, hyper));
    //     self.eindex += 1;

    //     if let Some(s) = s {
    //         for v in &s {
    //             self.vdata.get_mut(v).unwrap().out_edges.insert(index);
    //         }
    //     }

    //     if let Some(t) = t {
    //         for v in &t {
    //             self.vdata.get_mut(v).unwrap().in_edges.insert(index);
    //         }
    //     }

    //     index
    // }

    fn set_inputs(&mut self, inputs: Vec<usize>) {
        self.inputs = inputs;
    }

    fn set_outputs(&mut self, outputs: Vec<usize>) {
        self.outputs = outputs;
    }

    // // fn out_edges(&self, v: usize) -> Option<&.
    // fn add_vertex(&mut self, x: f32, y: f32, value: &str, name: VertexIndex) -> VertexIndex {
    //     if name == -1 {
    //         let v = self.vindex;
    //         self.vindex += 1;
    //         self.vdata.insert(v, VData {
    //             x,
    //             y,
    //             value: value.to_string(),
    //             in_edges: HashSet::new(),
    //             out_edges: HashSet::new(),
    //             in_indices: HashSet::new(),
    //             out_indices: HashSet::new(),
    //             highlight: false,
    //         });
    //         v
    //     } else {
    //         let v = name;
    //         let max_index = name.max(self.vindex);
    //         self.vindex = max_index + 1;
    //         self.vdata.insert(v, VData {
    //             x,
    //             y,
    //             value: value.to_string(),
    //             in_edges: HashSet::new(),
    //             out_edges: HashSet::new(),
    //             in_indices: HashSet::new(),
    //             out_indices: HashSet::new(),
    //             highlight: false,
    //         });
    //         v
    //     }
    // }

    // fn add_edge(&mut self, s: Vec<VertexIndex>, t: Vec<VertexIndex>, value: &str, x: f32, y: f32, fg: &str, bg: &str, hyper: bool, name: EdgeIndex) -> EdgeIndex {
    //     if name == -1 {
    //         let e = self.eindex;
    //         self.eindex += 1;
    //         self.edata.insert(e, EData {
    //             s,
    //             t,
    //             value: value.to_string(),
    //             x,
    //             y,
    //             fg: fg.to_string(),
    //             bg: bg.to_string(),
    //             hyper,
    //             highlight: false,
    //         });
    //         for v in &s {
    //             self.vdata.get_mut(v).unwrap().out_edges.insert(e);
    //         }
    //         for v in &t {
    //             self.vdata.get_mut(v).unwrap().in_edges.insert(e);
    //         }
    //         e
    //     } else {
    //         let e = name;
    //         let max_index = name.max(self.eindex);
    //         self.eindex = max_index + 1;
    //         self.edata.insert(e, EData {
    //             s,
    //             t,
    //             value: value.to_string(),
    //             x,
    //             y,
    //             fg: fg.to_string(),
    //             bg: bg.to_string(),
    //             hyper,
    //             highlight: false,
    //         });
    //         for v in &s {
    //             self.vdata.get_mut(v).unwrap().out_edges.insert(e);
    //         }
    //         for v in &t {
    //             self.vdata.get_mut(v).unwrap().in_edges.insert(e);
    //         }
    //         e
    //     }
    // }

    // fn remove_vertex(&mut self, v: VertexIndex, strict.
}
