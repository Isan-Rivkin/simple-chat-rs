use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE : usize = 32;

fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}
pub fn run() {
    let server = TcpListener::bind(LOCAL).expect("Listener: failed to bind");
    // to be able to listen to msgs non blocking 
    server.set_nonblocking(true).expect("failed to initialize non blocking mode");
    // allow multiple clients 
    let mut clients = vec![];
    let (tx,rx) = mpsc::channel::<String>();
    loop{
        if let Ok((mut socket,addr)) = server.accept(){
            println!("Client {} connected ", addr);
            let tx = tx.clone();
            // clone the socket so that we can push it into our thread 
            clients.push(socket.try_clone().expect("failed to clone clinet"));
            thread::spawn(move || loop {
                let mut buff = vec![0;MSG_SIZE];
                
                match socket.read_exact(&mut buff){
                    Ok(_)=>{
                        let msg = buff.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                        let msg =  String::from_utf8(msg).expect("invalid utf8 msg");

                        println!("{}: {:?}",addr,msg );
                        tx.send(msg).expect("failed to send msg to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) =>{
                        println!("closing connection with {}",addr );
                        break;
                    }
                }
                sleep();
            });
        }
        if let Ok(msg) = rx.try_recv(){
            clients = clients.into_iter().filter_map(|mut client|{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE,0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();
    }
}
