#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;

extern crate uuid; 

// the order of featur matters

use serde::json;
use uuid::Uuid;
use std::collections::HashMap;

fn new_id() -> String {
     Uuid::new_v4().to_string()
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Item {
     id   :     String,
     title:     Option<String>,
     completed : bool,
     order  :   u32,
     text   :   Option<String>,
}

/*
fn fqdn (s: String) -> String {
    s
}
*/

impl Item  {

fn new() -> Item {
     let id = new_id();
     Item { id : id, completed: false, order: 0, title: None, text: None }
}
fn url(&self) -> String {
    let host = "http://todo-backend-rust.herokuapp.com";
    let path = "todo";
    let id = self.id.to_owned();
    format!("{}/{}/{}", host, path, id)
}

fn update(&mut self, item: Item) -> &mut Item {
    // pattern math?
     self.title = item.title;
     self.completed = item.completed;
     self.order = item.order;
     self.text = item.text;
     self
}

}

