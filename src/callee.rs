use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Result};
use regex::Regex;


pub struct FunctionNode {
    name: String,
    callees: Vec<Rc<RefCell<FunctionNode>>>,
    curr_depth: usize,
}

impl FunctionNode {
    pub fn new(name: String, curr_depth_para:usize) -> FunctionNode {
        FunctionNode {
            name,
            callees: vec![],
            curr_depth: curr_depth_para,
        }
    }
    fn add_callee(&mut self, child: FunctionNode) {
        self.callees.push(Rc::new(RefCell::new(child)));
    }
}


pub struct Callee {
    fn_hash: RefCell<HashMap<String, usize>>,
    no_def_fn: RefCell<HashSet<String>>,
    pub source: Vec<String>,
    pub root: Rc<RefCell<FunctionNode>>,
    fn_brackets_count: usize,
    yaml_file_path: String,
}

impl Callee {
    pub fn new(root_fn_name:String, output_file_name:String) -> Callee {
        Callee {
            fn_hash: RefCell::new(HashMap::new()),
            no_def_fn: RefCell::new(HashSet::new()),
            source: Vec::new(),
            root: Rc::new(RefCell::new(FunctionNode::new(root_fn_name, 0))),
            fn_brackets_count: 0,
            yaml_file_path: output_file_name, // デフォルトの出力先はyaml_output/call_graph.yaml
        }
    }

    fn fn_def_start_condition(&self, fn_name: &str, line: &str) -> bool {
        if line.starts_with(format!("{}(", fn_name).as_str()) {
            return true;
        }
        if line.contains(format!(" {}(", fn_name).as_str()) {
            return true;
        }
        if line.starts_with(format!("*{}(", fn_name).as_str()) {
            return true;
        }
        if line.contains(format!(" *{}(", fn_name).as_str()) {
            return true;
        }
        false
    }

    fn is_comment_line(&self, line: usize) -> bool {
        let line_content = self.source[line].trim();
        if line_content.starts_with("//") || line_content.starts_with("/*") || line_content.starts_with("*") || line_content.ends_with("*/") {
            return true;
        }
        false
    }

    fn is_function_definition(&self, index: usize, fn_name: &str) -> bool {
        let mut line_num = index;
        let mut is_param_line = false;
        let mut fin_param = false;
        while line_num<=self.source.len() {
            let line = self.source[line_num].trim();
            if !is_param_line {
                if self.is_comment_line(line_num) {
                    return false;
                }
                if self.fn_def_start_condition(fn_name, line) {
                    is_param_line = true;
                    if line.ends_with(")") {
                        fin_param = true;
                    }else if line.contains(")") && line.ends_with("{") {
                        return true;
                    }
                }
                else {
                    return false;
                }
            }else if !fin_param {
                if line.ends_with(",") {
                    line_num += 1;
                    continue;
                }
                if line.ends_with(")") {
                    fin_param = true;
                } else {
                    return false;
                }
            }else{
                if line == ("{") {
                    return true;
                }
                return false;
            }
            line_num += 1;
        }
        false
    }

    // nameの関数の中身を調べて、そこから呼び出してる関数を子として登録する
    // 子のノードに対しては,nameとcurr_depthだけ設定する
    // 他のノードとの子の重複はとりあえずチェックしない
    // ある関数の定義が何行目にあるかのハッシュテーブルを参照する
    fn add_callee_fn(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_name = fn_node.borrow().name.clone();
        let fn_name_clone = fn_name.clone();
        let mut fn_line:i64 = -1;
        {
            let no_def_fn = self.no_def_fn.borrow_mut();
            if no_def_fn.contains(&fn_name) {
                return;
            }
            let mut fn_hash = self.fn_hash.borrow_mut();
            if let Some(&line) = fn_hash.get(&fn_name) {
                fn_line = line as i64;
            } else {
                for (i, _line) in self.source.iter().enumerate() {
                    if self.is_function_definition(i, &fn_name) {  
                        fn_hash.insert(fn_name, i );
                        fn_line = i as i64;
                        break;
                    }
                }
            }
        }
        if fn_line != -1 {
            self.add_child_node(fn_node, fn_line as usize);
        }
        else {
            let mut no_def_fn = self.no_def_fn.borrow_mut();
            println!("{} is not defined", fn_name_clone);
            no_def_fn.insert(fn_name_clone);
        }
    }

    fn is_fn_call_line(&self, line: usize) -> bool {
        let line_content = self.source[line].trim();
        // コメントのチェック
        if line_content.starts_with("//") || line_content.starts_with("/*") ||line_content.starts_with("*") || line_content.ends_with("*/") {
            return false;
        }
        // 関数呼び出しの基本的なチェック
        let re = Regex::new(r"\S\(").unwrap(); // 空白ではない文字の後に"("がくるパターン
        if re.is_match(line_content) && line_content.contains(")") {
            return true;
        }
        false
    }
    

    fn find_fn_call(&mut self, line: usize) -> Vec<String> {
        let mut fn_names = Vec::new(); // 関数名を格納するためのベクター
        let re = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\(").unwrap();

    
        if self.is_fn_call_line(line) {
            let line_content = self.source[line].trim(); // 前置空白を取り除く
            for cap in re.captures_iter(line_content) {
                // 正規表現に一致する全ての部分をイテレート
                let mut fn_name = cap[0].to_string();
                fn_name.pop(); // 末尾の"("を取り除く
                fn_names.push(fn_name);
            }
        }
        fn_names
    }
    
    

    fn add_child_node(&mut self, fn_node: &Rc<RefCell<FunctionNode>>, line: usize) {
        // line行目から始まる関数において、呼び出してる関数を子として登録する
        // 子のノードに対しては,nameとcurr_depthだけ設定する

        let mut curr_line = line+1;
        // 関数定義の部分をスキップ
        while curr_line < self.source.len() {
            let trimmed_line = self.source[curr_line].trim();
            if trimmed_line.starts_with("{") {
                self.fn_brackets_count += 1;
                curr_line += 1;
                break;
            }
            curr_line += 1;
        }
        
        // curr_line行目から順番にfind_fn_call()を使用して関数呼び出しを探す
        // fn_brackets_countが0になるまで繰り返す,
        while self.fn_brackets_count > 0 {
            let line_content = self.source[curr_line].trim();
    
            // 閉じ括弧"}"を含むかどうかでカウンタを更新
            if line_content.contains("}") {
                self.fn_brackets_count -= line_content.matches("}").count();
            }

            // 開き括弧"{"を含むかどうかでカウンタを更新（ネストされたブロックを考慮）
            if line_content.contains("{") {
                self.fn_brackets_count += line_content.matches("{").count();
            }
    
            // 関数呼び出しのチェック
            let fn_names = self.find_fn_call(curr_line);
            if !fn_names.is_empty() {
                for fn_name in fn_names {
                    // fn_nodeのロックを取得して子ノードを追加
                    let mut fn_node_locked = fn_node.borrow_mut();
                    let curr_depth_buf = fn_node_locked.curr_depth;
                    let fn_name_clone = fn_name.clone();
                    println!("parent={}, child={}, curr_depth={}", fn_node_locked.name, fn_name_clone, curr_depth_buf+1);
                    fn_node_locked.add_callee(FunctionNode::new(fn_name, curr_depth_buf+1));
                }
            }
    
            curr_line += 1;
        }
 
    }

    // 次に自分の関数があるかチェックして、あれば自分が呼び出してる関数を子として全て格納
    // 全ての子に対して再帰的にこの関数を呼び出す
    fn search_c_fn(&mut self, depth:usize, fn_node: &Rc<RefCell<FunctionNode>>) {
        self.add_callee_fn(&Rc::clone(fn_node));
        // 子に対して再帰的にこの関数を呼び出す (深さもチェック)
        let fn_node_locked = fn_node.borrow_mut();
        for child in fn_node_locked.callees.iter() {
            if depth > fn_node_locked.curr_depth+1 {
                self.search_c_fn(depth, &Rc::clone(child));
            }
        }
    }

    // Call Graphを生成するための関数
    pub fn generate_call_graph(&mut self, depth: usize)  {
        let root_clone = Rc::clone(&self.root);
        self.search_c_fn(depth, &root_clone);
    }

    #[allow(dead_code)]
    fn print_node_test(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_node_locked = fn_node.borrow_mut();
        println!("name={}, curr_depth={}", fn_node_locked.name, fn_node_locked.curr_depth);
        for child in fn_node_locked.callees.iter() {
            self.print_node_test(&Rc::clone(child));
        }
    }

    pub fn write_yaml(&mut self) -> Result<()> {
        let file_path = self.yaml_file_path.clone();
        let file = File::create(file_path)?;
        let mut writer = std::io::BufWriter::new(file);
        writeln!(writer, "<callee_graph>")?;

        self.write_node_yaml(&mut writer, &self.root, 0)
    }

    fn write_node_yaml(&self, writer: &mut impl Write, fn_node: &Rc<RefCell<FunctionNode>>, depth: usize) -> Result<()> {
        let fn_node_locked = fn_node.borrow();
        writeln!(writer, "{}{}: {}()", " ".repeat(depth * 4), fn_node_locked.curr_depth,fn_node_locked.name)?;
        
        for child in fn_node_locked.callees.iter() {
            self.write_node_yaml(writer, &Rc::clone(child), depth + 1)?;
        }

        Ok(())
    }

    pub fn output_yaml(&mut self) {
        self.write_yaml().expect("Failed to write YAML");
    }

}

