mod sample;
use sample::data2;
use sample::iterators;
use std::{io};
use crate::iterators::iterators::iterator_one;

#[allow(unused_variables)]

fn main() {

    let mut input=String::new();

    println!("say something");
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            println!("you said {}",input);

            let greeting=say_hello(&mut &input);
            println!("{}",greeting);
        },
        Err(e)=>{
            println!("something went wrong {}",e);
        }
    }

    let d="bimal";
    for i in 1..10{
        say_hi(d);
    }

    iterator_one();


}

fn say_hi(name :&str) {
    println!("hi {}", name);
    data2::say_bad_hello();
    sample::data::say_bad_hello2();
}

fn say_hello(name: &mut &String)->String {
    let greetings=format!("Hello {}",name);
    return greetings
}