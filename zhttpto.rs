//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;
use std::io::File;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut counter: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));

            let path_vec: ~[&str] = request_str.split(' ').collect();

            
            let response: ~str = 
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 </body></html>\r\n";

            unsafe {
                counter = counter + 1;
                let counter_resp: ~str = counter.to_str();
                stream.write(response.as_bytes());
                stream.write("Page visits: ".as_bytes());
                stream.write(counter_resp.as_bytes());

                match File::open(&Path::new(path_vec[1])) {
                    Some(file) => {
                        let mut reader = BufferedReader::new(file);
                        //reading from file
                        let file_bytes: ~[u8] = reader.read_to_end();

                        let newline: ~str = ~"<h1></h1></html>\n";
                        stream.write(newline.as_bytes());
                        stream.write("File contents:".as_bytes());
                        stream.write(newline.as_bytes());
                        stream.write(newline.as_bytes());
                        stream.write(file_bytes);
                    }
                    None =>{
                        println("Opening message.txt failed!");
                    }
                }
            }

            println!("Connection terminates.");
        }
    }
}
