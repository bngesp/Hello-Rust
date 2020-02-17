use std::collections::HashMap;
use std::string::ToString;
#[derive(Debug)]
pub struct WordCount{
 element: HashMap<String, u64>,
 nombre: u64
}

impl WordCount {
    pub(crate) fn new() -> WordCount {
        WordCount{element: HashMap::new(), nombre:0}
    }
    pub(crate) fn increment(&mut self, word: &str){
        let key: String = word.to_string();
        let count = self.element.entry(key).or_insert(0);
        *count+=1;
        self.nombre+=1;
    }

    pub(crate) fn display(self){
        for (k, v) in self.element.iter() {
            println!("{}: {}", k, v);
        }
        println!("le nombre de mot {}", self.nombre);
    }
}