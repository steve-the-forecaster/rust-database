use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpListener, path::Path, fs::{File, self}
};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
            
        println!("Request {:?}", http_request);
        if http_request[0].contains("GET") {
            let mut buf_writer = BufWriter::new(stream);
            
            let mut full_path = String::new();
            
            match File::open(Path::new(&String::from("path.txt"))) {
                Ok(mut file) => {
                    file.read_to_string(&mut full_path).expect("Couldn't open path");
                },
                Err(err) => {
                    println!("Couldn't open path {}", err)
                }
            }
            let metadata = fs::metadata(full_path.clone()).expect("Couldn't create buffer");
            let mut buffer = vec![0; metadata.len() as usize];
            
            File::open(full_path).expect("Incorrect path").read_to_end(&mut buffer).expect("Buffer overflow");
            
            buf_writer.write(buffer.leak()).expect("Couldn't send the buffer");
        } else if http_request[0].contains("POST") {
            for line in http_request {
                if line.contains("User") {
                    
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