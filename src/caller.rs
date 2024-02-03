use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Result};
use regex::Regex;

pub struct FunctionNode {
    name: String,
    callers: Vec<Rc<RefCell<FunctionNode>>>,
    curr_depth: usize,
}

impl FunctionNode {
    pub fn new(name: String, curr_depth_para:usize) -> FunctionNode {
        FunctionNode {
            name,
            callers: vec![],
            curr_depth: curr_depth_para,
        }
    }
    fn add_caller(&mut self, child: FunctionNode) {
        self.callers.push(Rc::new(RefCell::new(child)));
    }
}

pub struct Caller {
    fn_hash: RefCell<HashMap<String, Vec<usize>>>, // key: 関数名, value: その関数を呼び出してる関数の定義がある行番号
    no_used_fn: RefCell<HashSet<String>>,
    pub source: Vec<String>,
    pub root: Rc<RefCell<FunctionNode>>,
    yaml_file_path: String,
}

impl Caller {
    pub fn new(root_fn_name:String, output_file_name:String) -> Caller {
        Caller {
            fn_hash: RefCell::new(HashMap::new()),
            no_used_fn: RefCell::new(HashSet::new()),
            source: Vec::new(),
            root: Rc::new(RefCell::new(FunctionNode::new(root_fn_name, 0))),
            yaml_file_path: output_file_name,
        }
    }

    fn is_function_definition(&self, index: usize) -> bool {
        if index + 1 >= self.source.len() {
            return false;
        }
    
        let current_line = self.source[index].trim();
        let next_line = self.source[index + 1].trim();
        let next_next_line = self.source[index + 2].trim();
        let next_next_next_line = self.source[index + 3].trim();
    
        if current_line.contains("(") &&current_line.contains(")") && next_line == "{" {
            return true;
        }
        else if current_line.contains("(") && next_line.contains(")") && next_next_line == "{" {
            return true;
        }
        else {
            return false;
        }
    }

    fn extract_fn_name(&self, line: &str) -> String {
        let re = Regex::new(r"(\w+)\s*\(").unwrap();
        let caps = re.captures(line).unwrap();
        caps.get(1).unwrap().as_str().to_string()
    }

    fn fn_name_of_designated_line(&self, index: usize) -> String {
        let line = self.source[index].trim();
        self.extract_fn_name(&line)
    }

    // nameの関数を呼び出してる関数をcallersに追加
    fn add_caller_fn(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_name_paren=fn_node.borrow().name.clone()+ "(";
        let fn_name = fn_node.borrow().name.clone();
        let mut curr_fn: String = "".to_string();
        let mut curr_fn_line: usize = 0;
        let mut called_counter: usize = 0;
        // まずはhashにあるかチェック
        if self.no_used_fn.borrow().contains(&fn_name) {
            return;
        }
        if self.fn_hash.borrow().contains_key(&fn_name) {
            // hashのVecをすべて取り出して、callersに追加
            // hashには、その関数を呼び出してる関数の定義がある行番号が格納されている
            for line in self.fn_hash.borrow().get(&fn_name).unwrap() {
                let fn_name = self.fn_name_of_designated_line(*line);
                let mut fn_node_borrow = fn_node.borrow_mut();
                let curr_depth = fn_node_borrow.curr_depth;
                fn_node_borrow.add_caller(FunctionNode::new(fn_name, curr_depth + 1));
            }
            return;
        }
        // なければcのsourceを見て探す
        for (i, line) in self.source.iter().enumerate() {
            if self.is_function_definition(i) {
                curr_fn = self.extract_fn_name(&line);
                curr_fn_line = i;
                continue;
            }
            if line.contains(&fn_name_paren) {
                // in_what_fnをcallerとして登録
                let mut fn_node_borrow = fn_node.borrow_mut();
                let curr_depth = fn_node_borrow.curr_depth;
                fn_node_borrow.add_caller(FunctionNode::new(curr_fn.clone(), curr_depth + 1));
                // fn_hashに登録
                self.fn_hash.borrow_mut().entry(fn_name.clone()).or_insert(Vec::new()).push(curr_fn_line);
                called_counter += 1;
            }
        }
        if called_counter == 0 {
            self.no_used_fn.borrow_mut().insert(fn_name);
        }

    }

    // まずは深さチェック
    // 次に自分の関数があるかチェックして、あれば自分が呼び出してる関数を子として全て格納
    // 全ての子に対して再帰的にこの関数を呼び出す
    fn search_c_fn(&mut self, depth:usize, fn_node: &Rc<RefCell<FunctionNode>>) {
        {
            let fn_node_locked = fn_node.borrow_mut();
            if depth == fn_node_locked.curr_depth {
                return;
            }
        }
        self.add_caller_fn(&Rc::clone(fn_node));
        // 子に対して再帰的にこの関数を呼び出す
        let fn_node_locked = fn_node.borrow_mut();
        for child in fn_node_locked.callers.iter() {
            self.search_c_fn(depth, &Rc::clone(child));
        }
    }

    pub fn generate_call_graph(&mut self, depth: usize) {
        let root_clone = Rc::clone(&self.root);
        self.search_c_fn(depth, &root_clone);
    }

    #[allow(dead_code)]
    fn print_node_test(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_node_locked = fn_node.borrow_mut();
        println!("name={}, curr_depth={}", fn_node_locked.name, fn_node_locked.curr_depth);
        for child in fn_node_locked.callers.iter() {
            self.print_node_test(&Rc::clone(child));
        }
    }

    pub fn write_yaml(&mut self) -> Result<()> {
        let file_path = self.yaml_file_path.clone();
        let file = File::create(file_path)?;
        let mut writer = std::io::BufWriter::new(file);

        self.write_node_yaml(&mut writer, &self.root, 0)
    }

    fn write_node_yaml(&self, writer: &mut impl Write, fn_node: &Rc<RefCell<FunctionNode>>, depth: usize) -> Result<()> {
        let fn_node_locked = fn_node.borrow();
        writeln!(writer, "{}{}: {}()", " ".repeat(depth * 4), fn_node_locked.curr_depth,fn_node_locked.name)?;
        
        for child in fn_node_locked.callers.iter() {
            self.write_node_yaml(writer, &Rc::clone(child), depth + 1)?;
        }

        Ok(())
    }

    pub fn output_yaml(&mut self) {
        self.write_yaml().expect("Failed to write YAML");
    }

}