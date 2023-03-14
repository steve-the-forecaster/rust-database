use std::{io::{BufRead, Read, self, Write}, fs};

fn main() -> Result<(), io::Error> {
    // Anonymous address for now
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
        if http_request[0].contains("GET") {
            if let Ok(mut file) = fs::File::open(std::path::Path::new(&String::from("path.txt"))) {
                let mut path = String::new();
                
                file.read_to_string(&mut path).expect("path.txt didn't have a valid path");
                
                let mut buffer = vec![0; fs::metadata(path.clone()).expect("Couldn't create buffer").len() as usize];
                fs::File::open(path).expect("Incorrect path").read_to_end(&mut buffer).expect("Buffer overflow");
                
                io::copy(&mut file, &mut stream).expect("couldn't send file");
            } else {
                panic!("Couldn't read path.txt")
            }
        } else if http_request[0].contains("POST") {
            for line in http_request {
                if line.starts_with("User") {
                    let mut database = fs::File::open("database.json").expect("Couldn't open the data file");
                    
                    database.write(&[0]).expect("Couldn't write to the file");
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