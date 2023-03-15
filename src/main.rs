use std::{io::{BufRead, Read, self, Write}, fs};

fn main() -> Result<(), io::Error> {
    // loopback address for now
    let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = io::BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
            
        println!("Request {:?}", http_request);
        
        // No checks for validity yet since this isn't deployed
        // Will probably implement some hash verification but for now its not important
        if http_request[0].contains("GET") {
            // Checks if we can open the file
            if let Ok(mut file) = fs::File::open(std::path::Path::new(&String::from("path.txt"))) {
                let mut path = String::new();
                
                // Checks if the read action was successful
                if let Ok(_) = file.read_to_string(&mut path) {
                    io::copy(&mut fs::File::open(path).unwrap(), &mut stream).expect("couldn't send file");
                }
            }
        } else if http_request[0].contains("POST") {
            for line in http_request {
                if line.starts_with("User") {
                    fs::File::open("database.json").unwrap().write(&line.into_bytes()).expect("Couldn't write to the file");
                }
            }
        }
    }

    Ok(())
}

/*
fn split_string(character: char, string: &str) -> Vec<&str> {
    let mut chunks: Vec<&str> = vec![];

    let mut marker = 0;

    for (index, chr) in string.chars().enumerate() {
        if chr == character {
            chunks.push(&string[marker..index]);
            marker = index + 1;
        }
    }
    chunks.push(&string[marker..string.len()]);

    return chunks;
}
*/