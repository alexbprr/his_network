use std::collections::BTreeMap;
use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use anyhow::Error;

#[derive(Clone,Debug,Default,Serialize,Deserialize,Eq,PartialEq)]
pub enum LinkType {
    Infection,
    Infected,
    Differentiation,
    Production,
    Consume, 
    Replication, 
    Migration,
    Killing, 
    Phagocytosis,
    Apoptosis, 
    Decay,
    PositiveInteraction, 
    NegativeInteraction,
    Inhibition,
    #[default]None,
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,Eq,PartialEq)]
pub enum Sign {
    Negative = -1,
    #[default]None = 0, 
    Positive = 1, 
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,Eq,PartialEq)]
pub enum NodeType {
    #[default]Default = 0, 
    Interaction = 1,
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
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

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Edge {
    id: usize, 
    active: bool,
    src: usize, //source node 
    dest: usize, //destination node 
    signs: (Sign,Sign), //source and destination signs
    value: f64, //current value for the reaction 
    link_type: LinkType,
}

impl Edge {
    pub fn new(id: usize, src: usize, dest: usize, ltype: LinkType)-> Self {

        Self {            
            id: id,
            active: true,
            src: src,
            dest: dest,
            signs: (Sign::None,Sign::None),
            value: 0.0,
            link_type: ltype,
            //expression: MathExpression::new(),        
        }
    }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct Node {    
    pub id: usize,
    node_type: NodeType,
    active: bool,
    pub name: String,
    pub description: String,    
    input_links: Vec<usize>, //ids of the input edges 
    output_links: Vec<usize>, //ids of the output edges 
    //value: f64,
}

impl Node {
    pub fn new(id: usize, name: String)-> Self {

        Self {            
            id: id,
            node_type: NodeType::Default,
            active: true,
            name: name, 
            description: String::from(""),
            input_links: vec![],
            output_links: vec![],
            //value: 0.0,
        }
    }    

    pub fn set_node_type(&mut self, nt: NodeType){
        self.node_type = nt;
    }    
}

#[derive(Clone,Debug,Default,Serialize,Deserialize,PartialEq)]
pub struct BioNet {    
    name: String, 
    #[serde(skip_serializing,skip_deserializing)]
    gen_id: usize,
    pub node_map: BTreeMap<String,Node>, //dicionário 
    pub edge_map: BTreeMap<usize,Edge>, //dicionário
    pub parameters: HashMap<String,Parameter>,
}

impl BioNet {
    pub fn new(name: String)-> Self {

        Self {
            name: name,
            gen_id: 0,
            node_map: BTreeMap::new(),
            edge_map: BTreeMap::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn get_node_id(&self, name: String) -> usize {
        return self.node_map.get(&name).unwrap().id ;
    }

    pub fn get_node_name(&self, id: usize) -> String {

        for (name, node) in self.node_map.iter(){
            if node.id == id {
                return name.to_string() 
            }
        }

        return String::from("")
    }

    pub fn add_parameter(&mut self, name: String, value: f64){

        self.parameters.insert(name.clone(), Parameter::new(name, value));

    }

    pub fn create_parameters(&mut self, params: Vec<(String,f64)>){

        let values: Vec<(String, Parameter)> = 
                params
                    .iter()
                    .map(|v| (v.0.clone(), Parameter::new(v.0.clone(),v.1)))
                    .collect();

        self.parameters = HashMap::from_iter(values);
    }

    fn add_node(&mut self, node: Node){

        self.node_map.insert(node.name.clone(), node); 
        self.gen_id += 1;

    }

    fn add_edge(&mut self, edge: Edge){

        self.edge_map.insert(edge.id, edge); 
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

    pub fn create_edge(&mut self, src: String, dest: String, link_type: LinkType)-> Edge{

        let new_edge: Edge = Edge::new(self.gen_id, self.get_node_id(src.clone()), 
                                            self.get_node_id(dest.clone()), link_type);

        let Some(source_node) =  self.node_map.get_mut(&src) 
                else { 
                    println!("Node with id = {:?} not found!", src); return new_edge;
                };
        source_node.output_links.push(new_edge.id);

        let Some(dest_node) =  self.node_map.get_mut(&dest) 
                else { 
                    println!("Node with id = {:?} not found!", dest); return new_edge; 
                };

        dest_node.input_links.push(new_edge.id);

        self.add_edge(new_edge.clone()); 

        new_edge
    }

    pub fn add_node_to_interaction(&mut self, interaction: &Node, node: &Node, link_type: LinkType){

        let Some(interaction) = self.node_map.get(&interaction.name) 
                else {
                    println!("Node with id = {:?} not found!", interaction.id); return; 
                };
        let Some(node) = self.node_map.get(&node.name) 
                else {
                    println!("Node with id = {:?} not found!", node.id); return; 
                };

        //define the signs based on link type 
        self.create_edge(node.name.clone(), interaction.name.clone(), link_type);
    }

   /*  pub fn create_positive_interaction(&mut self, name: String, node1: &Node, node2: &Node, node3: &Node){
        let interaction = self.create_interaction(name);
        self.create_edge(node1.name.clone(), interaction.name.clone(), (Sign::None,Sign::Positive));
        self.create_edge(node2.name.clone(), interaction.name.clone(), (Sign::None,Sign::Positive));
        self.create_edge(interaction.name.clone(), node3.name.clone(), (Sign::None,Sign::Positive));
    }

    pub fn create_negative_interaction(&mut self, name: String, node1: &Node, node2: &Node, node3: &Node){
        let interaction = self.create_interaction(name);
        self.create_edge(node1.name.clone(), interaction.name.clone(), (Sign::None,Sign::Positive));
        self.create_edge(node2.name.clone(), interaction.name.clone(), (Sign::None,Sign::Positive));
        self.create_edge(interaction.name.clone(), node3.name.clone(), (Sign::None,Sign::Negative));
    }

    pub fn create_differentiation_with_influence(&mut self, name: String, src: &Node, influence: &Node, dest: &Node, sign: Sign){
        let interaction = self.create_interaction(name);
        self.create_edge(src.name.clone(), interaction.name.clone(), (Sign::Negative,Sign::None));
        self.create_edge(influence.name.clone(), interaction.name.clone(), (Sign::None,sign));
        self.create_edge(interaction.name.clone(), dest.name.clone(), (Sign::None,Sign::Positive));
    }
    */
    
    fn get_nodes_with_input_link(&self, sign: Sign) -> Vec<usize>{
 
        let mut results: Vec<usize> = vec![];
        for (name, node) in self.node_map.iter(){

            if node.node_type == NodeType::Interaction {
                continue;
            }

            for input_link in node.input_links.iter(){                
 
                if self.edge_map[input_link].signs.1 == sign {
                    let id: &usize = &self.get_node_id(name.clone());
                    if results.contains(id) == false {
                        results.push(*id);
                    }
                }
            }
        }
        return results
    }

    pub fn get_nodes_with_positive_input_link(&self) -> Vec<usize> {
        return self.get_nodes_with_input_link(Sign::Positive)
    }

    pub fn get_nodes_with_negative_input_link(&self) -> Vec<usize> {
        return self.get_nodes_with_input_link(Sign::Negative)
    }

    pub fn get_nodes_without_positive_input_link(&self) -> Vec<usize> {
 
        let nodes_with_pos_links = self.get_nodes_with_positive_input_link();
        let mut results: Vec<usize> = vec![];

        for (name, node) in self.node_map.iter(){
            let id: &usize = &self.get_node_id(name.clone());
 
            if node.node_type == NodeType::Default && nodes_with_pos_links.contains(id) == false {
                results.push(*id);
            }
        }

        return results 
    }

    pub fn get_nodes_without_negative_input_link(&self) -> Vec<usize> {
 
        let nodes_with_neg_links = self.get_nodes_with_negative_input_link();
        let mut results: Vec<usize> = vec![];

        for (name, node) in self.node_map.iter(){
            let id: &usize = &self.get_node_id(name.clone());

            if node.node_type == NodeType::Default && nodes_with_neg_links.contains(id) == false {
                results.push(*id);
            }
        }

        return results 
    }

    pub fn get_nodes_without_output_links(&self) -> Vec<usize> {
 
        let mut results: Vec<usize> = vec![];

        for (name, node) in self.node_map.iter(){
            let id: &usize = &self.get_node_id(name.clone());
 
            if node.node_type == NodeType::Default && node.output_links.is_empty() {
                results.push(*id);
            }
        }
        return results
    } 

    pub fn get_nodes_with_least_number_of_inputs(&self) -> Vec<usize> { 
        unimplemented!()
    }

    pub fn get_nodes_with_least_number_of_outputs(&self) -> Vec<usize>{ 
        unimplemented!()
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