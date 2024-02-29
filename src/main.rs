#![allow(non_camel_case_types)] 
#![allow(non_snake_case)]
use graph::BioNet;
use crate::graph::Sign;
mod graph;

fn main() {
    let mut his_net: BioNet = BioNet::new(String::from("Immune system network"));
    let V = his_net.create_node(String::from("V"));
    let Ap = his_net.create_node(String::from("Ap"));
    let Apc = his_net.create_node(String::from("Apc"));
    let Thn = his_net.create_node(String::from("Thn"));
    let The = his_net.create_node(String::from("The"));

    his_net.create_positive_interaction(String::from("ap_activation"), &Ap, &V, &Apc);
    his_net.create_differentiation_with_influence(String::from("th_differentiation"), &Thn, &Apc, &The, Sign::Positive);

    println!("Network: {:#?}", his_net);
    his_net.save_net::<String>(String::from("./src/tests/his_network.json")).unwrap();

    let net = BioNet::load_net::<String>(String::from("./src/tests/his_network.json")).unwrap();
    println!("loaded network: {:#?}", net);

    if matches!(his_net, net){
        println!("the networks are equal!");
    }
}
