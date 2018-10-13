mod client;
mod network;

use network::connect as n_c;
use network::server;
use network::server::connect as s_c;

fn main() {
    println!("Hello, world!"); // Hello, world!
    println!("{:?}", client::connect()); // "This is client!"
    println!("{:?}", network::connect()); // "This is network!"
    println!("{:?}", network::server::connect()); // "This is server!"
    println!("{:?}", server::connect()); // "This is server!"
    println!("{:?}", n_c()); // "This is network!"
    println!("{:?}", s_c()); // "This is server!"
    println!("{:?}", network::server::super_connect()); // "This is network!"
}
