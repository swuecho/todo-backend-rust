#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
extern crate uuid; 

// the order of featur matters

use serde::json;
use std::collections::HashMap;
use self::item;

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


/*
type Todo map[string]*TodoItem

func (t Todo) All() []*TodoItem {
     items := []*TodoItem{}
     for _, item := range t {
     	 items = append(items, item)
	 }
	 return items
}

func (t Todo) Find(id string) *TodoItem {
     for _, item := range t {
     	 if item.Id == id {
	    	    return item
		    	   }
			   }

			   return nil
}

func (t Todo) Create(item TodoItem, fqdn func(string) string) *TodoItem {
     item.Id = newId()
     item.Url = fqdn("/todos/" + item.Id)
     t[item.Id] = &item
     return &item
}

func (t Todo) Update(id string, updatedItem TodoItem) *TodoItem {
     if item := t.Find(id); item != nil {
     	return item.Update(updatedItem)
	} else {
	  return nil
	  }
}

func (t Todo) DeleteAll() string {
     for k := range t {
     	 delete(t, k)
	 }
	 return ""
}

func (t Todo) Delete(id string) string {
     for k := range t {
     	 if k == id {
	      delete(t, k)
	      		}
			}
			return ""
}

*/
