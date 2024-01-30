mod analyze;

use analyze::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::rc::Rc;



fn main() {
    let args: Vec<String> = env::args().collect();
    print!("args: {:?}", args);
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        return;
    }

    // configファイルを読み込んで処理する
    let config_file = &args[1];
    let file = fs::File::open(config_file).expect("Failed to open config file");
    let mut lines = io::BufReader::new(file).lines();
    let c_source_file = lines.next().expect("Expected C source file path").expect("Failed to read line");
    let target_function = lines.next().expect("Expected target function name").expect("Failed to read line");
    let depth_str = lines.next().expect("Expected depth").expect("Failed to read line");
    let depth: usize = depth_str.parse().expect("Depth must be a number");


    // C言語のソースファイルを読み込んで処理する
    let c_file = fs::File::open(c_source_file).expect("Failed to open C source file");
    let reader = io::BufReader::new(c_file);
    let mut temp_lines: Vec<String> = Vec::new(); // 一時的なStringのVec
    for line in reader.lines() {
        match line {
            Ok(ln) => temp_lines.push(ln),
            Err(e) => println!("Error reading line: {}", e),
        }
    }


    // Call Graphを生成するための処理を呼び出す
    let mut parser = Parser::new(target_function.clone());
    parser.source = temp_lines.clone();
    parser.generate_call_graph(depth);
    let root_clone2 = Rc::clone(&parser.root);
    //output_yaml(root_clone2);
}

