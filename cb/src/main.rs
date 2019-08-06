extern crate crossbeam;

//use std::thread;
use std::collections::VecDeque;


fn main() {
    let people = [
        "Alice".to_string(),
        "Bob".to_string(),
        "Carol".to_string(),
        "Dave".to_string(),
        "Ellen".to_string(),
        "Frank".to_string(),
    ];

    let mut queue: VecDeque<String> = VecDeque::new();
//    let mut handles = Vec::new();
//    let mut queue = &mut queue.clone();

    crossbeam::scope(|scope| {
        for person in &people {
            let h = scope.spawn(|_| {
                queue.push_back(person.to_string());
            });
            h.join();
        }
    }).unwrap();

//    println!("{:?}", queue);
}
