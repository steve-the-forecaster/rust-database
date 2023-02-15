use std::{net::TcpListener, io::{Read, Write}, fs::File};

fn main() -> Result<(), std::io::Error>{
    let _base = vec![1];

    let path = std::path::Path::new(".\\table.txt");
    let listener = TcpListener::bind("127.0.0.1").unwrap();

    Ok(for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf: Vec<u8> = vec![];

        match stream.read(&mut buf) {
            Ok(_) => {
                let mut file = File::open(path).expect(&format!("Couldn't open {}", path.display()));
                let chunks = split_string(' ', buf.clone());

                file.write_all(format!("{}, {}, {}", chunks[0], chunks[1], chunks[2]).as_bytes()).expect(&format!("Couldn't write {}", path.display()));
            },
            Err(_) => println!("Couldn't read {}", path.display())
        }
    })
}

fn split_string(character: char, string: Vec<u8>) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];

    let mut i = 0;

    for (index, byte) in string.clone().into_iter().enumerate() {
        if byte as char == character {
            chunks.push(String::from_utf8(string[i..index].to_vec()).unwrap());
        }

        i += 1;
    }

    return chunks;
}