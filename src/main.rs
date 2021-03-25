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

    println!("{:?} {:?}", action, item);
}
