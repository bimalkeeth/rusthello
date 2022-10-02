pub mod iterators{
  pub fn iterator_one(){
      let numbers=[100,300,400];
      for number in numbers.iter(){
          println!("{}",number)
      }

  }

  pub fn vectors_return() ->Vec<i32>{
      let mut list:Vec<i32>=Vec::new();
      for i in 1..10 {
          list.push(i);
      }
      return list
  }

}