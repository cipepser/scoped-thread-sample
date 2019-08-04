use std::thread;
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
    let mut handles = Vec::new();

    for person in &people {
        let mut queue = queue.clone();
        handles.push(
            thread::spawn(move || {
                queue.push_back(person.to_string());
            })
        )
    }

    for h in handles {
        h.join().unwrap()
    }

    println!("{:?}", queue);
}
