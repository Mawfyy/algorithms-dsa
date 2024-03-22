mod hashmap;
mod linkedlist;

use hashmap::Hashmap;

fn main() {
    //HashMap implementation (for learning)
    let mut map = Hashmap::<String>::new(10);

    //Inserts
    map.insert("hola".to_string(), "miau".to_string());

    //Get the value from some key
    let result = map.get("hola".to_string());
    println!("{:?}", result);

    //Check if the key exists
    let has_value = map.exists("hola".to_string());
    println!("{:?}", has_value);

    //Remove the key
    map.remove("hola".to_string());

    let has_value = map.exists("hola".to_string());
    println!("{}", has_value);
}
