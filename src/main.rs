mod callee;
mod caller;

use callee::*;
use caller::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};



fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} {{caller or callee}} <config_file> [output_file]", args[0]);
        return;
    }
    // 引数が2つある場合、2つ目の引数をoutput_fileに格納
    let output_file = if args.len() == 3 {
        Some(&args[2])
    } else {
        None
    };

    // configファイルを読み込んで処理する
    let config_file = &args[1];
    let file = fs::File::open(config_file).expect("Failed to open config file");
    let mut lines = io::BufReader::new(file).lines();
    // 1行目からCソースファイルのパスを読み取る
    let c_source_files_line = lines.next().expect("Expected C source file paths").unwrap();
    let c_source_files: Vec<&str> = c_source_files_line.split(',').collect();
    // ファイルが0個の場合、エラーを表示
    if c_source_files.is_empty() {
        panic!("No C source files provided");
    }
    let target_function = lines.next().expect("Expected target function name").expect("Failed to read line");
    let depth_str = lines.next().expect("Expected depth").expect("Failed to read line");
    let depth: usize = depth_str.parse().expect("Depth must be a number");
    let mode = lines.next().expect("Expected mode").expect("Failed to read line");


    // C言語のソースファイルを読み込んで処理する
    let mut temp_lines: Vec<String> = Vec::new(); // 一時的なStringのVec
    for file_path in &c_source_files {
        let c_file = fs::File::open(file_path).expect("Failed to open C source file");
        let reader = io::BufReader::new(c_file);

        for line in reader.lines() {
            match line {
                Ok(ln) => temp_lines.push(ln),
                Err(e) => println!("Error reading line: {}", e),
            }
        }
    }


    // Call Graphを生成するための処理を呼び出す
    if mode == "callee" {
        fs::create_dir_all("yaml_output").expect("Failed to create yaml_output directory");
        let output_file_name = match output_file {
            Some(name) => format!("yaml_output/{}", name),
            None => "yaml_output/callee_graph.yaml".to_string(),
        };
        let mut callee = Callee::new(target_function.clone(), output_file_name);
        callee.source = temp_lines.clone();
        callee.generate_call_graph(depth);
        callee.output_yaml();
    }
    else if mode == "caller" {
        fs::create_dir_all("yaml_output").expect("Failed to create yaml_output directory");
        let output_file_name = match output_file {
            Some(name) => format!("yaml_output/{}", name),
            None => "yaml_output/caller_graph.yaml".to_string(),
        };
        let mut caller = Caller::new(target_function.clone(), output_file_name);
        caller.source = temp_lines.clone();
        caller.generate_call_graph(depth);
        caller.output_yaml();
    }
    else {
        eprintln!("Usage: {} {{caller or callee}} <config_file> [output_file]", args[0]);
        return;
    }
}

