# scoped-thread-sample

[Rust and Blockchain \- Speaker Deck](https://speakerdeck.com/osuke/rust-and-blockchain)を読んで、scoped threadが気になったので触ってみる。

## まずはstd::threadを使ったサンプルコードを作る

```zsh
❯ cargo new standard
```

```rust
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
```

これを実行すると以下のようにlifetimeのエラーが出る。

```zsh
❯ cargo run
   Compiling standard v0.1.0 (/Users/cipepser/.go/src/github.com/cipepser/scoped-thread-sample/standard)
warning: variable does not need to be mutable
  --> src/main.rs:14:9
   |
14 |     let mut queue: VecDeque<String> = VecDeque::new();
   |         ----^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: #[warn(unused_mut)] on by default

error[E0597]: `people` does not live long enough
  --> src/main.rs:17:19
   |
17 |     for person in &people {
   |                   ^^^^^^^
   |                   |
   |                   borrowed value does not live long enough
   |                   argument requires that `people` is borrowed for `'static`
...
32 | }
   | - `people` dropped here while still borrowed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: Could not compile `standard`.

To learn more, run the command again with --verbose.
```

## crossbeamを使ってみる

```zsh
❯ cargo new cb
     Created binary (application) `cb` package
```

`VeqDeque`使ってしまうのは一足飛びだったので少し戻る。

## std::threadに戻る

```rs
use std::thread;

fn main() {
    let people = [
        "Alice".to_string(),
        "Bob".to_string(),
        "Carol".to_string(),
        "Dave".to_string(),
        "Ellen".to_string(),
        "Frank".to_string(),
    ];

    let mut handles = Vec::new();

    for person in &people {
        handles.push(
            thread::spawn(move || {
                println!("{:?}", person);
            })
        )
    }

    for h in handles {
        h.join().unwrap()
    }
}
```

これを実行すると以下のようにlifetimeのエラーが出る。

```zsh
❯ cargo run
   Compiling standard v0.1.0 (/Users/cipepser/.go/src/github.com/cipepser/scoped-thread-sample/standard)
error[E0597]: `people` does not live long enough
  --> src/main.rs:17:19
   |
17 |     for person in &people {
   |                   ^^^^^^^
   |                   |
   |                   borrowed value does not live long enough
   |                   argument requires that `people` is borrowed for `'static`
...
32 | }
   | - `people` dropped here while still borrowed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: Could not compile `standard`.

To learn more, run the command again with --verbose.
```

## crossbeamを使ってみる

```rs
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

    crossbeam::scope(|scope| {
        for person in &people {
            let h = scope.spawn(move |_| {
                println!("{:?}", person);
            });
        }
    }).unwrap();
}
```

これを実行すると以下のようになる。

```zsh
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cb`
"Alice"
"Frank"
"Carol"
"Ellen"
"Dave"
"Bob"
```

エラーもないし、実行ごとに出力される順番も違う。

`crossbeam::thread`のシグネチャは以下のようになっている。

```rs
pub fn scope<'env, F, R>(f: F) -> thread::Result<R>
where
    F: FnOnce(&Scope<'env>) -> R,
{
// 中略
}
```

ライフライム`'env`をもたせている。


## References
- [Rust and Blockchain \- Speaker Deck](https://speakerdeck.com/osuke/rust-and-blockchain)