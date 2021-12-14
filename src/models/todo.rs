use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {

    pub uuid: String,

    pub name: String,
    pub content: String,

    pub is_done: bool,
}

impl Todo {

    pub fn new(name: String, content: String, is_done: bool) -> Self {
        Todo {
             uuid: generate_uuid(),
             name, 
             content, 
             is_done, 
            }
    }

    pub fn set_is_done(&mut self, is_done: bool) {
        self.is_done = is_done
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string();
    } 

    pub fn set_content(&mut self, content: &String) {
        self.content = content.to_string();
    }
    pub fn set_uuid(&mut self) {
        self.uuid = generate_uuid()
    }
    pub fn replace(&mut self, replace_with: Todo) {
        self.name = replace_with.name;
        self.content = replace_with.content;
        self.is_done = replace_with.is_done;
    }

}

fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}