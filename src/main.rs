mod callee;

use callee::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};



fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <config_file> [output_file]", args[0]);
        return;
    }
    // 引数が3つある場合、3つ目の引数をoutput_fileに格納
    let output_file = if args.len() == 3 {
        Some(&args[2])
    } else {
        None
    };

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
    fs::create_dir_all("yaml_output").expect("Failed to create yaml_output directory");
    let output_file_name = match output_file {
        Some(name) => format!("yaml_output/{}", name),
        None => "yaml_output/call_graph.yaml".to_string(),
    };
    let mut parser = Parser::new(target_function.clone(), output_file_name);
    parser.source = temp_lines.clone();
    parser.generate_call_graph(depth);
    parser.output_yaml();
}

