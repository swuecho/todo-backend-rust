extern crate uuid;
use uuid::Uuid;
fn new_id() -> String {
     Uuid::new_v4().to_string()
}
#[derive(Debug)]
struct Item {
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

fn main() {
     let it = Item::new();
     println!("{:?}", it);
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
