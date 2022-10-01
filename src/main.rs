use std::io;


fn main() {

    let mut input=String::new();

    println!("say something");
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            println!("you said {}",input)
        },
        Err(e)=>{
            println!("something went wrong {}",e);
        }
    }

    println!("Hello, world!");
}
