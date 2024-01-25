mod analyze;

use analyze::*;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};

// 依存関係解析のための関数や構造体をここに定義する
// ...



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        return;
    }

    let config_file = &args[1];
    let file = fs::File::open(config_file).expect("Failed to open config file");
    let mut lines = io::BufReader::new(file).lines();

    let c_source_file = lines.next().expect("Expected C source file path").expect("Failed to read line");
    let target_function = lines.next().expect("Expected target function name").expect("Failed to read line");
    let depth_str = lines.next().expect("Expected depth").expect("Failed to read line");
    let depth: usize = depth_str.parse().expect("Depth must be a number");

    let source_code = fs::read_to_string(&c_source_file)
        .expect("Failed to read C source file");

        let root = Arc::new(Mutex::new(FunctionNode::new(target_function)));
        let root_clone = Arc::clone(&root);
        generate_call_graph(&source_code, depth, root_clone);
        let root_clone2 = Arc::clone(&root);
        output_yaml(root_clone2);
}

