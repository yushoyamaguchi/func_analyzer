use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Result};
use regex::Regex;

pub struct FunctionNode {
    name: String,
    calls: Vec<Rc<RefCell<FunctionNode>>>,
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
        self.calls.push(Rc::new(RefCell::new(child)));
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