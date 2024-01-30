use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Result};


pub struct FunctionNode {
    name: String,
    calls: Vec<Rc<RefCell<FunctionNode>>>,
    curr_depth: usize,
}

impl FunctionNode {
    pub fn new(name: String, curr_depth_para:usize) -> FunctionNode {
        FunctionNode {
            name,
            calls: vec![],
            curr_depth: curr_depth_para,
        }
    }
    fn add_child(&mut self, child: FunctionNode) {
        self.calls.push(Rc::new(RefCell::new(child)));
    }
}


pub struct Parser {
    fn_hash: RefCell<HashMap<String, usize>>,
    pub source: Vec<String>,
    pub root: Rc<RefCell<FunctionNode>>,
    fn_brackets_count: usize,
    yaml_file_path: String,
}

impl Parser {
    pub fn new(root_fn_name:String, output_file_name:String) -> Parser {
        Parser {
            fn_hash: RefCell::new(HashMap::new()),
            source: Vec::new(),
            root: Rc::new(RefCell::new(FunctionNode::new(root_fn_name, 0))),
            fn_brackets_count: 0,
            yaml_file_path: output_file_name, // デフォルトの出力先はyaml_output/call_graph.yaml
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
    fn parse_c_fn(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_name = fn_node.borrow().name.clone();
        let mut fn_line:i64 = -1;
        {
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
            self.find_child(fn_node, fn_line as usize);
        }
    }

    fn is_fn_call_line(&self, line: usize) -> bool {
        let line_content = self.source[line].trim();
    
        // 一行コメントのチェック
        if line_content.starts_with("//") {
            return false;
        }
        // 複数行コメントのチェック（その行に収まる場合）
        if line_content.starts_with("/*") && line_content.ends_with("*/") {
            return false;
        }
        // 関数呼び出しの基本的なチェック
        if line_content.ends_with(";") && line_content.contains("(") && line_content.contains(")") {
            if !line_content.contains("=") && !line_content.contains("+") 
               && !line_content.contains("-") && !line_content.contains("*") 
               && !line_content.contains("/") && !line_content.contains("&&") 
               && !line_content.contains("||") {
                return true;
            }
        }
        false
    }
    

    fn find_fn_call(&mut self, line: usize)->Option<String> {
        // self.source[line]の中から関数呼び出しを探す
        // あればその関数名を返す
        // なければNoneを返す
        if self.is_fn_call_line(line) {
            let line_content = self.source[line].trim(); // 前置空白を取り除く
            if let Some(before_parentheses) = line_content.split("(").next() {
                // `split` の後、`next` を使って最初の要素を取得
                if let Some(fn_name) = before_parentheses.split_whitespace().last() {
                    // 空白で分割し、最後の要素を取得
                    return Some(fn_name.to_string());
                }
            }
        }
        None
    }

    fn find_child(&mut self, fn_node: &Rc<RefCell<FunctionNode>>, line: usize) {
        // line行目から始まる関数において、呼び出してる関数を子として登録する
        // 子のノードに対しては,nameとcurr_depthだけ設定する

        let mut curr_line = line+1;
        // line+1行目が"{"になっているか一応確認、なってたらline+1行目の括弧をfn_brackets_countに加算
        if self.source[curr_line].trim() == "{" {
            self.fn_brackets_count += 1;
            curr_line += 1;
        } else {
            println!("line={} , {}() is not implemented in this source code",line, fn_node.borrow().name);
            return;
        }
        // curr_line行目から順番にfind_fn_call()を使用して関数呼び出しを探す
        // fn_brackets_countが0になるまで繰り返す,
        while self.fn_brackets_count > 0 {
            let line_content = self.source[curr_line].trim();
    
            // 閉じ括弧のチェック
            if line_content == "}" {
                self.fn_brackets_count -= 1;
            }
    
            // 開き括弧のチェック（ネストされたブロックを考慮）
            if line_content == "{" {
                self.fn_brackets_count += 1;
            }
    
            // 関数呼び出しのチェック
            if let Some(fn_name) = self.find_fn_call(curr_line) {
                // fn_nodeのロックを取得して子ノードを追加
                let mut fn_node_locked = fn_node.borrow_mut();
                let fn_name_clone = fn_name.clone();
                let curr_depth_buf = fn_node_locked.curr_depth;
                fn_node_locked.add_child(FunctionNode::new(fn_name, curr_depth_buf+1));
                //println!("parent={}, child={}, curr_depth={}", fn_node_locked.name, fn_name_clone, curr_depth_buf+1);
            }
    
            curr_line += 1;
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
        self.parse_c_fn(&Rc::clone(fn_node));
        // 子に対して再帰的にこの関数を呼び出す
        let fn_node_locked = fn_node.borrow_mut();
        for child in fn_node_locked.calls.iter() {
            self.search_c_fn(depth, &Rc::clone(child));
        }
    }

    // Call Graphを生成するための関数
    pub fn generate_call_graph(&mut self, depth: usize)  {
        let depth = depth;
        let root_clone = Rc::clone(&self.root);
        self.search_c_fn(depth, &root_clone);
    }

    #[allow(dead_code)]
    fn print_node_test(&mut self, fn_node: &Rc<RefCell<FunctionNode>>) {
        let fn_node_locked = fn_node.borrow_mut();
        println!("name={}, curr_depth={}", fn_node_locked.name, fn_node_locked.curr_depth);
        for child in fn_node_locked.calls.iter() {
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
        
        for child in fn_node_locked.calls.iter() {
            self.write_node_yaml(writer, &Rc::clone(child), depth + 1)?;
        }

        Ok(())
    }

    pub fn output_yaml(&mut self) {
        /*// rootから順番に出力する
        let root_clone = Rc::clone(&self.root);
        self.print_node_test(&root_clone);*/
        self.write_yaml().expect("Failed to write YAML");
    }

}


