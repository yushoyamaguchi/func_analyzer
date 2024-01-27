mod analyze;

use analyze::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};



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
    let depth: i64 = depth_str.parse().expect("Depth must be a number");


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
    let  c_src_lines: Vec<&str> = temp_lines.iter().map(AsRef::as_ref).collect();


    // Call Graphを生成するための処理を呼び出す
    let mut parser = Parser::new();
    let root = Arc::new(Mutex::new(FunctionNode::new(target_function)));
    let root_clone = Arc::clone(&root);
    generate_call_graph(&c_src_lines, depth, root_clone, &mut parser);
    let root_clone2 = Arc::clone(&root);
    output_yaml(root_clone2);
}

