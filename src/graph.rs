use std::collections::BTreeMap;
use serde::Serialize;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use anyhow::Error;

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub enum Sign {
    Negative = -1,
    #[default]None = 0, 
    Positive = 1, 
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub enum NodeType {
    #[default]Default = 0, 
    Interaction = 1,
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct Reaction {
    pub expr_text: String,
    pub numeric_expr: String,
    pub inputs: Vec<String>, //cada variavél do lado esquerdo é um input 
    pub outputs: Vec<(String,i32)>, //cada variavél do lado direito é um output 
    pub rate: f64,
}

impl Reaction {

    pub fn new() -> Self {
        Self {
            expr_text: String::from(""),
            numeric_expr: String::from(""),
            inputs: vec![],
            outputs: vec![],
            rate: 0.0,
        }
    }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: f64,
}

impl Parameter {
    pub fn new(name: String, value: f64)-> Self {
        Self {
            name: name,
            value: value,
        }
    }
}  

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct Edge {
    id: usize, 
    active: bool,
    src: usize, //source node 
    dest: usize, //destination node 
    signs: (Sign,Sign), //source and destination signs
    value: f64, //current value for the reaction 
    expression: Reaction,
    parameters: Vec<String>, 
}

impl Edge {
    pub fn new(id: usize, src: usize, dest: usize, signs: (Sign,Sign))-> Self {
        Self {            
            id: id,
            active: true,
            src: src,
            dest: dest,
            signs: signs,            
            value: 0.0,
            expression: Reaction::new(),
            parameters: vec![],
        }
    }

    pub fn build_expression(){
        //store expression in self.expression.expr_text 
    }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct Node {    
    pub id: usize,
    node_type: NodeType,
    active: bool,
    pub name: String,
    input_links: Vec<usize>, //ids of the input edges 
    output_links: Vec<usize>,
    value: f64,
}

impl Node {
    pub fn new(id: usize, name: String)-> Self {
        Self {            
            id: id,
            node_type: NodeType::Default,
            active: true,
            name: name, 
            input_links: vec![],
            output_links: vec![],
            value: 0.0,
        }
    }

    pub fn set_node_type(&mut self, nt: NodeType){
        self.node_type = nt;
    }
}

//BioNet type: directed 
#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct BioNet {    
    name: String, 
    #[serde(skip_serializing,skip_deserializing)]
    gen_id: usize,
    node_list: BTreeMap<usize,Node>,
    edge_list: BTreeMap<usize,Edge>,
    parameters: BTreeMap<String,Parameter>,
}
//the BioNet current state is the set of values of all nodes 

impl BioNet {
    pub fn new(name: String)-> Self {
        Self {
            name: name,
            gen_id: 0,
            //g_size: 0,
            node_list: BTreeMap::new(),
            edge_list: BTreeMap::new(),
            parameters: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node){
        self.node_list.insert(self.gen_id, node); 
        self.gen_id += 1;
    }

    pub fn add_edge(&mut self, edge: Edge){
        self.edge_list.insert(self.gen_id, edge); 
        self.gen_id += 1;
    }    

    pub fn create_node(&mut self, name: String)-> Node{
        let new_node: Node = Node::new(self.gen_id, name);
        self.add_node(new_node.clone());
        new_node
    }

    pub fn create_interaction(&mut self, name: String)-> Node{
        let mut new_node: Node = Node::new(self.gen_id, name);
        new_node.set_node_type(NodeType::Interaction);
        self.add_node(new_node.clone());
        new_node
    }        

    pub fn create_edge(&mut self, src: usize, dest: usize, signs: (Sign,Sign))-> Edge{
        let new_edge: Edge = Edge::new(self.gen_id, src, dest, signs);
        self.node_list.get_mut(&src).unwrap().output_links.push(new_edge.id);
        self.node_list.get_mut(&dest).unwrap().input_links.push(new_edge.id);
        self.add_edge(new_edge.clone()); 
        new_edge
    }

    pub fn add_node_to_interaction(&mut self, interaction: &Node, node: &Node, signs: (Sign,Sign)){
        //match self.node_list.get(&interaction.id){
        self.create_edge(node.id, interaction.id, signs);        
    }

    pub fn create_positive_interaction(&mut self, name: String, node1: &Node, node2: &Node, node3: &Node){
        let interaction = self.create_interaction(name);
        self.create_edge(node1.id, interaction.id, (Sign::None,Sign::Positive));
        self.create_edge(node2.id, interaction.id, (Sign::None,Sign::Positive));
        self.create_edge(interaction.id, node3.id, (Sign::None,Sign::Positive));
    }

    pub fn create_negative_interaction(&mut self, name: String, node1: &Node, node2: &Node, node3: &Node){
        let interaction = self.create_interaction(name);
        self.create_edge(node1.id, interaction.id, (Sign::None,Sign::Positive));
        self.create_edge(node2.id, interaction.id, (Sign::None,Sign::Positive));
        self.create_edge(interaction.id, node3.id, (Sign::None,Sign::Negative));
    }

    pub fn create_differentiation_with_influence(&mut self, name: String, src: &Node, influence: &Node, dest: &Node, sign: Sign){
        let interaction = self.create_interaction(name);
        self.create_edge(src.id, interaction.id, (Sign::Negative,Sign::None));
        self.create_edge(influence.id, interaction.id, (Sign::None,sign));
        self.create_edge(interaction.id, dest.id, (Sign::None,Sign::Positive));
    }

    pub fn get_node(){
        unimplemented!()
    }

    pub fn get_edge(){
        unimplemented!()
    }

    pub fn build_node_expr(){

    }

    pub fn save_net<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<(),Error> {
        let file: File = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        };
        let writer: BufWriter<File> = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn load_net<P: AsRef<Path>>(path: P) -> anyhow::Result<BioNet,Error> {        
        let file: File = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        };
        let reader: BufReader<File> = BufReader::new(file);
        let json: Result<BioNet, serde_json::Error> = serde_json::from_reader(reader);
        match json {
            Ok(f) => Ok(f),
            Err(e) => return Err(e.into()),
        }
    }

}