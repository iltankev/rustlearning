/* use rand::*; // 0.8.5

fn main() {
    let mut randchar: char = rand::random::<char>();

    loop {
       if randchar == '𥆿' {
        break
       } else {
        println!("{}",randchar);
       };
       randchar = rand::random::<char>()
    }
}
 */
 use rand::{distributions::Alphanumeric, Rng}; // 0.8

fn main() {
    let mut s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
        while s != "0" {
            println!("{}", s);
            s = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1)
            .map(char::from)
            .collect();
        }
}