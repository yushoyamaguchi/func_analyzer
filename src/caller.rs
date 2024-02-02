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
    fn_hash: RefCell<HashMap<String, Vec<usize>>>, // key: 関数名, value: 関数が呼び出されてる行番号のVec
    no_used_fn: RefCell<HashSet<String>>,
    pub source: Vec<String>,
    pub root: Rc<RefCell<FunctionNode>>,
    fn_brackets_count: usize,
    yaml_file_path: String,
}

impl Caller {
    pub fn new(root_fn_name:String, output_file_name:String) -> Caller {
        Caller {
            fn_hash: RefCell::new(HashMap::new()),
            no_used_fn: RefCell::new(HashSet::new()),
            source: Vec::new(),
            root: Rc::new(RefCell::new(FunctionNode::new(root_fn_name, 0))),
            fn_brackets_count: 0,
            yaml_file_path: output_file_name,
        }
    }

    pub fn generate_call_graph(&mut self, depth: usize) {
        
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