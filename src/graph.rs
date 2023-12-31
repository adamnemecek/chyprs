use std::collections::{
    HashMap,
    HashSet,
    VecDeque,
};
use std::iter::Iterator;

#[derive(Debug)]
struct GraphError;

#[derive(Clone, Debug)]
pub struct VData<T> {
    value: Option<T>,
    x: f32,
    y: f32,
    highlight: bool,
    in_edges: HashSet<usize>,
    out_edges: HashSet<usize>,
    in_indices: HashSet<usize>,
    out_indices: HashSet<usize>,
}

impl<T> VData<T> {
    fn new(
        x: f32,
        y: f32,
        value: impl Into<Option<T>>,
    ) -> Self {
        Self {
            value: value.into(),
            x,
            y,
            highlight: false,
            in_edges: HashSet::new(),
            out_edges: HashSet::new(),
            in_indices: HashSet::new(),
            out_indices: HashSet::new(),
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

///
/// The data assocaited with a single edge
///
#[derive(Clone, Debug)]
pub struct EData<T> {
    value: Option<T>,
    highlight: bool,
    x: f32,
    y: f32,
    s: Vec<usize>,
    t: Vec<usize>,
    fg: Option<String>,
    bg: Option<String>,
    hyper: bool,
}

impl<T> EData<T> {
    fn new(
        s: Vec<usize>,
        t: Vec<usize>,
        value: impl Into<Option<T>>,
        x: f32,
        y: f32,
        fg: Option<String>,
        bg: Option<String>,
        hyper: bool,
    ) -> Self {
        Self {
            value: value.into(),
            highlight: false,
            x,
            y,
            s,
            t,
            fg,
            bg,
            hyper,
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn s(&self) -> &Vec<usize> {
        &self.s
    }

    pub fn t(&self) -> &Vec<usize> {
        &self.t
    }
    ///
    /// Returns the number of 'units' of width the box should have to display nicely.
    ///
    /// The simple rule is if both inputs and outputs are <= 1, draw as a small (size 1) box, otherwise
    /// draw as a larger (size 2) box.
    ///
    fn box_size(&self) -> usize {
        if self.s.len() <= 1 && self.t.len() <= 1 {
            1
        } else {
            2
        }
    }
}

///
/// A hypergraph with boundaries
///
/// This is the main data structure used by Chyp. It represents a directed hypergraph (which we call simply a "graph")
/// as two dictionaries for vertices and (hyper)edges, respectively. Each vertex is associated with a `VData`
/// object and edge edge with an `EData` object, which stores information about adjacency, position, label,
/// etc.
///
/// The particular flavor of hypergraphs we use associate to each hyperedge a list of source vertices and a list
/// of target vertices. The hypergraph itself also has a list of input vertices and a list of output vertices,
/// which are used for sequential composition and rewriting.
///

#[derive(Clone, Debug)]
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

    ///
    /// Returns a graph with a single hyperedge and given number of inputs/outputs
    ///
    /// :param value: The label for the hyperedge
    /// :param arity: The number of input vertices connected to the source of the edge
    /// :param coarity: The number of output vertices connected to the target of the edge
    /// :param fg: An optional foregraph color, given as a 6-digit RGB hex code
    /// :param bg: An optional background color, given as a 6-digit RGB hex code
    ///
    pub fn gen<'a>(
        value: impl Into<Option<T>>,
        arity: usize,
        coarity: usize,
        fg: impl Into<Option<&'a str>>,
        bg: impl Into<Option<&'a str>>,
    ) -> Self {
        let mut g = Self::new();

        let inputs: Vec<_> = (0..arity)
            .map(|i| {
                g.add_vertex(
                    -1.5,
                    i as f32 - (arity as f32 - 1.0) / 2.0,
                    None,
                )
            })
            .collect();

        let outputs: Vec<_> = (0..coarity)
            .map(|i| {
                g.add_vertex(
                    1.5,
                    i as f32 - (coarity as f32 - 1.0) / 2.0,
                    None,
                )
            })
            .collect();

        g.add_edge(
            inputs.clone(),
            outputs.clone(),
            value.into(),
            None,
            None,
            fg,
            bg,
            None,
            None,
        );
        g.set_inputs(inputs);
        g.set_outputs(outputs);

        g
    }

    ///
    /// Returns a graph corresponding to the given permutation
    ///
    /// This takes a permution, given as a list [x0,..,x(n-1)], which is interpreted as the permutation { x0 -> 0, x1 -> 1, ..., x(n-1) -> n-1 }.
    /// It produces a graph consisting just of vertices, where input xj is mapped to the same vertex as output j, representing an identity
    /// wire connecting input xj to output j.
    ///
    /// Note this is one of two reasonable conventions for specifying a permutation as a list of numbers. This one has the property, e.g.
    /// for graphs aj : 0 -> 1, we have: (a0 * a1 * a2) >> perm([2, 0, 1]) = a2 * a0 * a1.
    ///
    /// :param p: A permutation, given as an n-element list of integers from 0 to n-1.
    ///
    pub fn perm(p: Vec<usize>) -> Self {
        let mut g = Self::new();

        // inputs = [g.add_vertex(0, i - (size-1)/2) for i in range(size)]
        let l = p.len();
        let inputs: Vec<_> = (0..l)
            .map(|i| {
                g.add_vertex(
                    0.0,
                    i as f32
                        - (l as f32 - 1.0) / 2.0 as f32,
                    None,
                )
            })
            .collect();

        let outputs: Vec<_> =
            (0..l).map(|i| inputs[p[i]]).collect();

        g.set_inputs(inputs);
        g.set_inputs(outputs);
        g
    }

    ///
    /// Returns a graph corresponding to the identity map
    ///
    /// This graph has a single vertex which is both an input and an output.
    ///
    pub fn identity() -> Self {
        let mut g = Self::new();
        let v = g.add_vertex(0.0, 0.0, None);
        g.set_inputs(vec![v]);
        g.set_outputs(vec![v]);
        g
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
        value: impl Into<Option<T>>,
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
        let mut succ = HashSet::default();

        let mut current = VecDeque::from_iter(vs);

        while let Some(v) = current.pop_front() {
            for e in self.out_edges(v) {
                for v1 in self.target(*e) {
                    if !succ.contains(v1) {
                        current.push_back(*e);
                        succ.insert(*v1);
                    }
                }
            }
        }

        succ
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

    ///
    /// Add an edge to the graph
    ///
    /// :param s:     A list of source vertices
    /// :param t:     A list of target vertices
    /// :param value: The value carried by this edge (typically a string)
    /// :param x:     The X coordinate to draw the box representing this hyperedge
    /// :param y:     The Y coordinate
    /// :param hyper: This is a hint to tell the GUI how to draw this (hyper)edge. If set to
    ///                 False, ideally it should be drawn simply as a line connected two vertices
    ///                 rather than as a box. (Currently not implemented.)
    /// :param name:  An optional name. If this is set to -1, set the name automatically.
    ///
    pub fn add_edge<'a>(
        &mut self,
        s: Vec<usize>,
        t: Vec<usize>,
        value: impl Into<Option<T>>,
        x: impl Into<Option<f32>>,
        y: impl Into<Option<f32>>,
        fg: impl Into<Option<&'a str>>,
        bg: impl Into<Option<&'a str>>,
        hyper: impl Into<Option<bool>>,
        name: impl Into<Option<usize>>,
    ) -> usize {
        let e;
        if let Some(name) = name.into() {
            e = name;
            self.eindex = name.max(self.eindex) + 1;
        } else {
            e = self.eindex;
            self.eindex += 1;
        }

        for v in t.iter() {
            let c = self.vdata.get_mut(&v).unwrap();
            c.in_edges.insert(e);
        }

        for v in s.iter() {
            let c = self.vdata.get_mut(&v).unwrap();
            c.out_edges.insert(e);
        }

        self.edata.insert(
            e,
            EData::new(
                s,
                t,
                value,
                x.into().unwrap_or_default(),
                y.into().unwrap_or_default(),
                fg.into().map(|x| x.to_owned()),
                bg.into().map(|x| x.to_owned()),
                hyper.into().unwrap_or(true),
            ),
        );

        e
    }

    fn set_inputs(&mut self, inputs: Vec<usize>) {
        self.inputs = inputs;
    }

    fn set_outputs(&mut self, outputs: Vec<usize>) {
        self.outputs = outputs;
    }

    pub fn inputs(&self) -> &Vec<usize> {
        unimplemented!()
    }

    pub fn outputs(&self) -> &Vec<usize> {
        unimplemented!()
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
