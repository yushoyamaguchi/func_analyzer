use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;


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


pub struct Parser {
    fn_hash: RefCell<HashMap<String, i64>>,
    pub source: Vec<String>,
    pub root: Arc<Mutex<FunctionNode>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            fn_hash: RefCell::new(HashMap::new()),
            source: Vec::new(),
        }
    }

    fn is_function_definition(&self, index: usize, fn_name: &str) -> bool {
        if index + 1 >= self.source.len() {
            return false;
        }
    
        let current_line = self.source[index].trim();
        let next_line = self.source[index + 1].trim();
    
        current_line.contains(&format!("{}(", fn_name)) && current_line.contains(")") && next_line == "{"
    }

    // nameの関数の中身を調べて、そこから呼び出してる関数を子として登録する
    // 子のノードに対しては,nameとcurr_depthだけ設定する
    // 他のノードとの子の重複はとりあえずチェックしない
    // ある関数の定義が何行目にあるかのハッシュテーブルを参照する
    fn parse_c_fn(&mut self, fn_node: &Arc<Mutex<FunctionNode>>) {
        let fn_name = fn_node.lock().unwrap().name.clone();
        let mut fn_line:i64 = 0;
        {
            let mut fn_hash = self.fn_hash.borrow_mut();


            if let Some(&line) = fn_hash.get(&fn_name) {
                println!("Function '{}' found in line {}", fn_name, line);
                fn_line = line;
            } else {
                for (i, line) in self.source.iter().enumerate() {
                    if self.is_function_definition(i, &fn_name) {  
                        println!("Function '{}' found in line {}", fn_name, i );
                        fn_hash.insert(fn_name, i as i64);
                        fn_line = i as i64;
                        break;
                    }
                }
            }
        }
        self.find_child(fn_node, fn_line);
    }

    fn parse_c_root_fn(&mut self) {
        //let fn_name = fn_node.lock().unwrap().name.clone();
        let fn_name=self.root.lock().unwrap().name.clone();
        let mut fn_line:i64 = 0;
        {
            let mut fn_hash = self.fn_hash.borrow_mut();


            if let Some(&line) = fn_hash.get(&fn_name) {
                println!("Function '{}' found in line {}", fn_name, line);
                fn_line = line;
            } else {
                for (i, line) in self.source.iter().enumerate() {
                    if self.is_function_definition(i, &fn_name) {  
                        println!("Function '{}' found in line {}", fn_name, i );
                        fn_hash.insert(fn_name, i as i64);
                        fn_line = i as i64;
                        break;
                    }
                }
            }
        }
        self.find_child(&self.root, fn_line);
    }

    fn find_child(&mut self, fn_node: &Arc<Mutex<FunctionNode>>, line: i64) {
        // line行目から始まる関数において、呼び出してる関数を子として登録する
        // 子のノードに対しては,nameとcurr_depthだけ設定する

        
    }

    // まずは深さチェック
    // 次に自分の関数があるかチェックして、あれば自分が呼び出してる関数を子として全て格納
    // 全ての子に対して再帰的にこの関数を呼び出す
    fn search_c_fn(&mut self, depth:i64, fn_node: &Arc<Mutex<FunctionNode>>) {
        let fn_node_locked = fn_node.lock().unwrap();
        if depth == fn_node_locked.curr_depth {
            return;
        }
        self.parse_c_fn(&fn_node);
        // 子に対して再帰的にこの関数を呼び出す
        let fn_node_locked = fn_node.lock().unwrap();
        for child in fn_node_locked.calls.iter() {
            self.search_c_fn(depth, &Arc::clone(child));
        }
    }

    // Call Graphを生成するための関数
    pub fn generate_call_graph(&mut self, depth: i64, root: Arc<Mutex<FunctionNode>>)  {
        let depth = depth;
        self.search_c_fn(depth, &root);
        
    }

}





// Call GraphをYAML形式で出力する関数
pub fn output_yaml(root: Arc<Mutex<FunctionNode>>) {
    // YAML形式での出力ロジックをここに実装
    print!("output_yaml() is not implemented yet");
}