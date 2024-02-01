use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

//Database connection
const DB_URL: &str = env!("DATABASE_URL");


//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function 
fn main() {
    //Set up database connection
    if let Err(_) = set_database() {
        println!("Error connecting to database");
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }

}

//db setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    ")?;
    Ok(())
}

fn get_id(request: &str) -> &str {
    request.split("/").nth(n).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap_or_default())
}

//handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    
    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(std::str::from_utf8(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("Options") =>(OK_RESPONSE.to_string(), "".to_string()),
                r if r.starts_with("POST /api/rust/users") => handle_post_request(r),
                
            }
        }
    }
}