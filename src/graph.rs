use std::collections::{
    HashMap,
    HashSet,
};
use std::iter::Iterator;

#[derive(Debug)]
struct GraphError;

#[derive(Clone, Debug)]
pub struct VData<T> {
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

    pub fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct EData<T> {
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

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn s(&self) -> &Vec<usize> {
        &self.s
    }

    pub fn t(&self) -> &Vec<usize> {
        &self.t
    }


    fn box_size(&self) -> usize {
        if self.s.len() <= 1 && self.t.len() <= 1 {
            1
        } else {
            2
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn vertex_data(&self, v: usize) -> &VData<T> {
        // self.vdata.get(&v)
        &self.vdata[&v]
    }

    pub fn edge_data(&self, e: usize) -> &EData<T> {
        // self.edata.get(&e)
        &self.edata[&e]
    }

    pub fn in_edges(&self, v: usize) -> &HashSet<usize> {
        // self.vdata.get(&v).map(|data| &data.in_edges)
        &self.vdata[&v].in_edges
    }

    pub fn out_edges(&self, v: usize) -> &HashSet<usize> {
        // self.vdata.get(&v).map(|data| &data.out_edges)
        &self.vdata[&v].out_edges
    }

    pub fn in_indices(&self, v: usize) -> &HashSet<usize> {
        &self.vdata[&v].in_indices
    }

    pub fn out_indices(&self, v: usize) -> &HashSet<usize> {
        // self.vdata.get(&v).map(|data| &data.out_indices)
        &self.vdata[&v].out_indices
    }

    pub fn source(&self, e: usize) -> &Vec<usize> {
        unimplemented!()
    }

    pub fn target(&self, e: usize) -> &Vec<usize> {
        unimplemented!()
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

    pub fn is_input(&self, v: usize) -> bool {
        unimplemented!()
        //
    }

    pub fn is_output(&self, v: usize) -> bool {
        unimplemented!()
        //
    }

    pub fn is_boundary(&self, v: usize) -> bool {
        self.is_input(v) || self.is_output(v)
        //
    }

    pub fn successors(
        &self,
        vs: impl Iterator<Item = usize>,
    ) -> HashSet<usize> {
        unimplemented!()
    }

    pub fn merge_vertices(&self, v: usize, w: usize) {
        unimplemented!()
    }

    pub fn explode_vertex(
        &self,
        v: usize,
    ) -> (Vec<usize>, Vec<usize>) {
        unimplemented!()
    }

    pub fn insert_id_after(
        &self,
        v: usize,
        reverse: bool,
    ) -> usize {
        unimplemented!()
    }

    pub fn tensor(&self, other: Self) {
        unimplemented!()
    }

    pub fn compose(&self, other: Self) {
        unimplemented!()
    }

    pub fn highlight(
        &mut self,
        vertices: HashSet<usize>,
        edges: HashSet<usize>,
    ) {
        unimplemented!()
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
