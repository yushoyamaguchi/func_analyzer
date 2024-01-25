use std::sync::{Arc, Mutex};

pub struct FunctionNode {
    name: String,
    calls: Vec<FunctionNode>,
}

impl FunctionNode {
    pub fn new(name: String) -> FunctionNode {
        FunctionNode {
            name,
            calls: vec![],
        }
    }
}



// Call Graphを生成するための関数
pub fn generate_call_graph(source: &str, depth: usize, root: Arc<Mutex<FunctionNode>>)  {
    let src_c_file = source;
    let depth = depth;

    
}



// Call GraphをYAML形式で出力する関数
pub fn output_yaml(root: Arc<Mutex<FunctionNode>>) {
    // YAML形式での出力ロジックをここに実装
    print!("output_yaml() is not implemented yet");
}