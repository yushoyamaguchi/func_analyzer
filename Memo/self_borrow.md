# selfの借用をするとき
```
    fn fnB(&mut self){
    	self.name=RefCell::new("BB".to_string());
    }
    fn fnA(&mut self){

    	{
            let mut var=self.name.borrow_mut(); //name: RefCell<String>
        }
    	self.fnB();
    }
```
fnBがmut selfを借用しているので、かっこでvarのスコープを限定しなければコンパイルできない