// for input
use std::io;

// structure of item
struct TodoItem{
    id: u64,
    title:String,
    completed:bool
}

// todo list
struct TodoList{
    items: Vec<TodoItem>
}

impl TodoList{
    fn new()->TodoList{
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title:String){
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem{
            id,
            title:title.clone(),
            completed:false
        };
        self.items.push(new_item);
        println!("Added {}",title);
    }

    fn list_items(&self){
        if self.items.is_empty(){
            println!("Todo List is empty")
        }else{
            println!("Your ToDo List:");
            for item in &self.items{
                let status = if item.completed { "[X]" } else {"[ ]"};
                println!("{} {} - {}",status,item.id,item.title);
            }
        } 
    }

    fn complete_item(&mut self, id:u64){
        if let Some(item) = self.items.iter_mut().find(|i| i.id ==id){
            item.completed = true;
            println!("Completed {}",item.title);
        }else{
            println!("Item with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop{
        println!("1. Add Item");
        println!("2. List Items");
        println!("3. Complete Item");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse()  {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1=>{
                println!("Enter the title of todo: ");
                let mut title = String::new();

                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add_item(title);
            }
            2=>{
                todo_list.list_items();
            }
            3=>{
                println!("Enter the ID of the completed item:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read");

                let id:u64 = match id.trim().parse() {
                    Ok(num)=>num,
                    Err(_)=>continue,
                };

                todo_list.complete_item(id);
            } 
            4=>{
                println!("Exiting The Program....");
                break;
            }

            _=>{
                println!("Invalid Choice");
            }
        }
    }
}
