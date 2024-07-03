use hurl_core::ast::JsonValue;
use hurl_core::parser;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let usage = "usage: jsonperf [parse-serde|parse-hurl|read-hurl|read-bytes] FILE";
    let args = env::args().collect::<Vec<_>>();
    let bench = args.get(1).expect(usage);
    let filename = args.get(2).expect(usage);

    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    match bench.as_str() {
        "parse-serde" => parse_with_serde_json(&content),
        "parse-hurl" => parse_with_hurl_core(&content),
        "read-hurl" => read_with_hurl_core(&content),
        "read-bytes" => read_with_bytes_reader(&content),
        _ => panic!("{usage}"),
    }
}

fn parse_with_hurl_core(data: &str) {
    let mut reader = parser::Reader::new(data);
    let data = parser::parse_json(&mut reader).unwrap();
    let JsonValue::List { elements, .. } = data else {
        panic!("invalid data");
    };
    println!("{}", elements.len());
}

fn read_with_hurl_core(data: &str) {
    let mut reader = parser::Reader::new(data);
    while reader.read().is_some() {}
    println!("{}", reader.is_eof())
}

fn read_with_bytes_reader(data: &str) {
    let mut reader = Reader::new(data);
    while reader.read().is_some() {}
    println!("{}", reader.is_eof())
}

fn parse_with_serde_json(data: &str) {
    let v: Value = serde_json::from_str(data).unwrap();
    let Value::Array(elements) = v else {
        panic!("invalid data");
    };
    println!("{}", elements.len());
}

/// Bytes reader
struct Reader<'input> {
    index: usize,
    bytes: &'input [u8],
}

impl<'input> Reader<'input> {
    fn new(text: &'input str) -> Self {
        Reader {
            index: 0,
            bytes: text.as_bytes(),
        }
    }

    fn read(&mut self) -> Option<u8> {
        if self.index >= self.bytes.len() {
            return None;
        }
        let c = self.bytes[self.index];
        self.index += 1;
        Some(c)
    }

    fn is_eof(&self) -> bool {
        self.index >= self.bytes.len()
    }
}
