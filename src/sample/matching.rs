
#[derive(Debug)]
pub enum Suit {
    Heart,
    Club,
    Diamond,
    Spade
}

pub fn print_choice(choice:Suit){
    match choice {
        Suit::Heart =>{println!("{:?}","this is heart")}
        Suit::Spade => { println!("{:?}","this is spade");}
        Suit::Club =>{println!("{:?}","this is club")}
        Suit::Diamond =>{println!("{:?}","this is diamond")}
    }
}