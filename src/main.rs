mod sample;
use sample::data2;
use sample::iterators;
use std::io;
use crate::iterators::iterators::{iterator_one, tuple_show, vectors_return};
use sample::structures;
use sample::matching;
use crate::matching::Suit::{Club, Diamond, Heart, Spade};


#[allow(unused_variables)]

fn main() {

    let mut input=String::new();

    println!("say something");
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            println!("you said {}",input);

            let greeting=say_hello(&mut &input);
            println!("{:?}",greeting);
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

    let vecs=vectors_return();

    for v in vecs.iter(){

        let print=|val:i32|{println!("{}",val *5);};
        print(*v);

    }

    tuple_show();
    structures::show_struct();

    matching::print_choice(Heart);
    matching::print_choice(Diamond);
    matching::print_choice(Spade);
    matching::print_choice(Club);
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