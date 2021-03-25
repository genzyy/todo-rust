use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    /**
     * The function here takes the mutable self variable and a key.
     * The & means the reference to the variable meaning if we change something
     * the change needs to be global or the address where the OG variable is stored.
     * The below concept is known as BORROW in Rust.
     */
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    /**
     * The -> means the return type of the function that we use here.
     * We save the contents of the tasks as task: completed status
     * in a local .txt file saved as "db.txt".
     */

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    /**
     * An alternative approach for below function.
     *for entries in content.lines() {
        // split and bind values
        let mut values = entries.split('\t');
        let key = values.next().expect("No Key");
        let val = values.next().expect("No Value");
        // insert them into HashMap
        map.insert(String::from(key), bool::from_str(val).unwrap());
    }
     */

    /**
     * The new() uses serde crate for reading json file.
     * then it reads using from_reader() and then maps
     * the contents as map of Todo.
     * If somehow the file is not found or encountered
     * we show a panic error.
     */

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    /**
     * The complete function takes the key(task) and then
     * changes or flips the complete state of that particular
     * task.
     */

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }
}

fn main() {
    /*
     * We used std::env function to get the arguments passed during the compilation
     * and then we use nth() to get the args by indices. The args().nth(0)
     * is the program itself and that's why we start from index 1.
     */

    let action = std::env::args()
        .nth(1)
        .expect("Please specify an action here!");

    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialization of db failed somehow");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Task saved successfully!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("The given task: {} not found!", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}
