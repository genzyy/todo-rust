use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    /**
     * The function here takes the mutable self variable and a key.
     * The & means the reference to the variable meaning if we change something
     * the change needs to be global or the address where the OG variable is stored.
     * The above concept is known as BORROW in Rust.
     */
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    /**
     * The -> means the return type of the function that we use here.
     * We save the contents of the tasks as task: completed status
     * in a local .txt file saved as "db.txt".
     */

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record: str = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        return std::fs::write("db.txt", content);
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

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Task saved successfully!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    println!("{:?} {:?}", action, item);
}
