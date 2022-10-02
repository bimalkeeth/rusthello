pub mod iterators{
  pub fn iterator_one(){
      let numbers=[100,300,400];
      for number in numbers.iter(){
          println!("{}",number)
      }

  }
}