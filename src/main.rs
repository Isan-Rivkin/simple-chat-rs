use std::env;

mod server;
mod client;

fn main() {
    let args : Vec<String> = env::args().collect();
    let is_server = args[1].clone();
    if is_server == "server" {
        println!("running server" );
        server::run();
    }else{
        println!("running client" );
        client::run();
    }
    
}
