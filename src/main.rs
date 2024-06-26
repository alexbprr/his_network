#![allow(non_camel_case_types)] 
#![allow(non_snake_case)]
use bionet::BioNet;
use crate::bionet::Sign;
mod bionet;

fn main() {    
    let mut his_net: BioNet = BioNet::new(String::from("Immune system network"));
    let params: Vec<(String,f64)> = vec![(String::from("k1"),0.05),(String::from("k2"),0.05),(String::from("k3"),0.05),
                                        (String::from("k4"),0.05),(String::from("k5"),0.05)];
    his_net.create_parameters(params);

    let V = his_net.create_node(String::from("V"));
    let Ap = his_net.create_node(String::from("Ap"));
    let Apc = his_net.create_node(String::from("Apc"));
    let Thn = his_net.create_node(String::from("Thn"));
    let The = his_net.create_node(String::from("The"));
    let Tkn = his_net.create_node(String::from("Tkn"));
    let Tke = his_net.create_node(String::from("Tke"));

   /* his_net.create_positive_interaction(String::from("ap_activation"), &Ap, &V, &Apc);
    his_net.create_differentiation_with_influence(String::from("th_differentiation"), &Thn, &Apc, &The, Sign::Positive);
    his_net.create_edge(Apc.name, V.name, (Sign::None,Sign::Negative));*/

    print!("nodes with positive input link: ");
    for id in his_net.get_nodes_with_positive_input_link(){
        print!("{:#?}, ", his_net.get_node_name(id));
    }
    println!();

    print!("nodes with negative input link: ");
    for id in his_net.get_nodes_with_negative_input_link(){
        print!("{:#?}, ", his_net.get_node_name(id));
    }
    println!();

    print!("nodes without positive input link: ");
    for id in his_net.get_nodes_without_positive_input_link(){
        print!("{:#?}, ", his_net.get_node_name(id));
    }
    println!();

    print!("nodes without negative input link: ");
    for id in his_net.get_nodes_without_negative_input_link(){
        print!("{:#?}, ", his_net.get_node_name(id));
    }
    println!();

    print!("nodes without output links: ");
    for id in his_net.get_nodes_without_output_links(){
        print!("{:#?}, ", his_net.get_node_name(id));
    }
    println!();
    
    
    println!("Network: {:#?}", his_net);
    his_net.save_net::<String>(String::from("./src/tests/his_network.json")).unwrap();

    //let net = BioNet::load_net::<String>(String::from("./src/tests/his_network.json")).unwrap();
}
