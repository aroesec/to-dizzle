use std::env;

fn main() {
    let todos = store_todos();
    get_todos(todos[1])
    // let terminal_args: Vec<String> = env::args().collect();
}

fn store_todos() -> Vec {
    let mut vector = Vec::new();
    let added_todos: Vec<String> = env::args().collect();

    for args in added_todos {
        println!("{}", args);  
        vector.push(args)
    }
    return vector
}
 


fn get_todos(todos:Vec<T>) {
    //grab todos and display in order
    println!("here are your todos")
}
