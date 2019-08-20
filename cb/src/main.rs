extern crate crossbeam;

use std::thread;
use std::collections::VecDeque;
use std::time::Duration;

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

    crossbeam::scope(|scope| {
        for person in &people {
//            let mut queue = queue.clone();
            scope.spawn(move |_| {
                println!("{:?}", person);
                queue.push_back("1".to_owned());
//                queue.push_back(person.clone());
            });
//            h.join();
        }
        scope.spawn(move |_| {
            thread::sleep(Duration::from_secs(1));
            println!("{:?}", queue);
        });
    }).unwrap();
}
