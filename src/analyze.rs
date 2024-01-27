use std::collections::HashMap;
use std::sync::{Arc, Mutex};


pub struct FunctionNode {
    name: String,
    calls: Vec<Arc<Mutex<FunctionNode>>>,
    curr_depth: i64,
    is_existed: bool,
}

pub struct Parser {
    fn_hash: Mutex<HashMap<String, usize>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            fn_hash: Mutex::new(HashMap::new()),
        }
    }

    // nameの関数の中身を調べて、そこから呼び出してる関数を子として登録する
    // 子のノードに対しては,nameとcurr_depthだけ設定する
    // 他のノードとの子の重複はとりあえずチェックしない
    // ある関数の定義が何行目にあるかのハッシュテーブルを参照する
    fn parse_c_fn(&mut self, source: &Vec<&str>, fn_node: &Arc<Mutex<FunctionNode>>) {
        let fn_name = fn_node.lock().unwrap().name.clone();
        let mut fn_hash = self.fn_hash.lock().unwrap();

        if let Some(&line) = fn_hash.get(&fn_name) {
            println!("Function '{}' found in line {}", fn_name, line);
        } else {
            for (i, line) in source.iter().enumerate() {
                if line.contains(&format!("{}(", fn_name)) {
                    println!("Function '{}' found in line {}", fn_name, i + 1);
                    fn_hash.insert(fn_name, i + 1);
                    break;
                }
            }
        }
    }


}


impl FunctionNode {
    pub fn new(name: String) -> FunctionNode {
        FunctionNode {
            name,
            calls: vec![],
            curr_depth: 0,
            is_existed: false,
        }
    }
}




// まずは深さチェック
// 次に自分の関数があるかチェックして、あれば自分が呼び出してる関数を子として全て格納
// 全ての子に対して再帰的にこの関数を呼び出す
fn search_c_fn(source: &Vec<&str>, depth:i64, fn_node: &Arc<Mutex<FunctionNode>>, parser: &mut Parser) {
    let fn_node_locked = fn_node.lock().unwrap();
    if depth == fn_node_locked.curr_depth {
        return;
    }
    parser.parse_c_fn(source, &fn_node);
    // 子に対して再帰的にこの関数を呼び出す
    let fn_node_locked = fn_node.lock().unwrap();
    for child in fn_node_locked.calls.iter() {
        search_c_fn(source, depth, &Arc::clone(child), parser);
    }
}


// Call Graphを生成するための関数
pub fn generate_call_graph(source: &Vec<&str>, depth: i64, root: Arc<Mutex<FunctionNode>>, parser: &mut Parser)  {
    let src_c_file = source;
    let depth = depth;
    search_c_fn(source, depth, &root, parser);
    
}



// Call GraphをYAML形式で出力する関数
pub fn output_yaml(root: Arc<Mutex<FunctionNode>>) {
    // YAML形式での出力ロジックをここに実装
    print!("output_yaml() is not implemented yet");
}