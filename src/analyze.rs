use std::sync::{Arc, Mutex};

pub struct FunctionNode {
    name: String,
    calls: Vec<Arc<Mutex<FunctionNode>>>,
    curr_depth: i64,
    is_existed: bool,
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

// nameの関数の中身を調べて、そこから呼び出してる関数を子として登録する
// 子のノードに対しては,nameとcurr_depthだけ設定する
// 他のノードとの子の重複はとりあえずチェックしない
fn parse_c_fn(source: &str, fn_node: &Arc<Mutex<FunctionNode>>) {
    // C言語の関数をパースするロジックをここに実装
    print!("parse_c_fn() is not implemented yet");
}


// まずは深さチェック
// 次に自分の関数があるかチェックして、あれば自分が呼び出してる関数を子として全て格納
// 全ての子に対して再帰的にこの関数を呼び出す
fn search_c_fn(source: &str, depth:i64, fn_node: &Arc<Mutex<FunctionNode>>) {
    let fn_node_locked = fn_node.lock().unwrap();
    if depth == fn_node_locked.curr_depth {
        return;
    }
    parse_c_fn(source, fn_node);
    // 子に対して再帰的にこの関数を呼び出す
    let fn_node_locked = fn_node.lock().unwrap();
    for child in fn_node_locked.calls.iter() {
        search_c_fn(source, depth, &Arc::clone(child));
    }
}


// Call Graphを生成するための関数
pub fn generate_call_graph(source: &str, depth: i64, root: Arc<Mutex<FunctionNode>>)  {
    let src_c_file = source;
    let depth = depth;

    let root_fn_name = root.lock().unwrap().name.clone();
    
}



// Call GraphをYAML形式で出力する関数
pub fn output_yaml(root: Arc<Mutex<FunctionNode>>) {
    // YAML形式での出力ロジックをここに実装
    print!("output_yaml() is not implemented yet");
}