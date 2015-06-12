use serde::json;
use std::collections::HashMap;
use item;

type TODO<'a> = HashMap<String,&'a Item>;

fn main() {
     let it = Item::new();
     println!("{:?}",json::to_string(&it).unwrap());
     let mut todo = TODO::new();
     todo.insert(it.id.to_owned(), &it);
     println!("{:?}",json::to_string(&todo).unwrap());
     println!("{:?}", todo);
     let url = it.url();
     println!("{}", url);
}

