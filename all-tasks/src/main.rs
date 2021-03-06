use std::{collections::HashMap, fs::read_to_string};

fn main() {

    //DAY 4
    print_hello_world();

    let greetings = "Hello world".to_string();
    println!("{}", greetings);

    greet("Hello TOO".to_string());

    greet(greetings);

    //DAY 5
    overshadow_and_mutable();
    ownership();
    borrow();
    borrow_mut_variable();

    //DAY 7
    if_else();
    add_numbers(4,5);
}


fn greet(target: String) {
    println!("{}", target);
}

fn print_hello_world(){
    println!("Hello world!");
}


fn overshadow_and_mutable() {
    let mut mutable = 1;
    println!("{}", mutable);

    mutable = 4; // Notice this isn't a number.

    println!("{}", mutable);

    let overshadow = "a";

    println!("{}", overshadow);

    let overshadow = 44;

    println!("{}", overshadow);
}

fn ownership() {
    let source = read_to_string("./README.md").unwrap();
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);
}

fn borrow(){
    let source = read_to_string("./README.md").unwrap();
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);

    let files_ref = &files;
    let files_ref2 = &files;

    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);
}

fn borrow_mut_variable(){
    let source = read_to_string("./README.md").unwrap();
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);

    let files_ref = &mut files;

    needs_mutable_ref(files_ref); 
    //


    let files_ref2 = &mut files;

   
    needs_mutable_ref(files_ref2);
}

fn needs_mutable_ref(map: &mut HashMap<&str, String>) {}


fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:?}", map)
}

fn if_else(){
    let apples = 6;
    let message = if apples > 10 {
        "Lots of apples"
    } else if apples > 4 {
        "A few apples"
    } else {
        "Not many apples at all"
    };

    println!("{}", message) // prints "A few apples"
}

fn add_numbers(left: i64, right: i64) -> i64 {
    left + right  // ??? Notice no semi-colon
}