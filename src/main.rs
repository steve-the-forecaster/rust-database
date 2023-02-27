use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::Path, iter::Enumerate
};

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("path.txt");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        for (index, line) in http_request.iter().enumerate() {
            if line.starts_with("Username") {
                add_entry(path, line.into_bytes());
            }
        }

        println!("Request:  {:?}\nContent: {:?}", http_request, http_request.get(3));
    }

    Ok(())
}

fn add_entry(path: &Path, entry: Vec<u8>) {
    let mut file = File::open(path).expect(&format!("Couldn't open {}", path.display()));
    let chunks = split_string(' ', entry.clone());
    println!("{:?}", String::from_utf8(entry));

    file.write_all(format!("{}, {}, {}", chunks[0], chunks[1], chunks[2]).as_bytes())
        .expect(&format!("Couldn't write {}", path.display()));
}

fn split_string(character: char, string: Vec<u8>) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];

    let mut marker = 0;

    for (index, byte) in string.clone().into_iter().enumerate() {
        if byte as char == character {
            chunks.push(String::from_utf8(string[marker..index].to_vec()).unwrap());
            
            marker = index;
        }
    }

    return chunks;
}
