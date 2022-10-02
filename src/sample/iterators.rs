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

  pub fn tuple_show() {
      let mut  person:(&str,i64,bool)=("Bimal",56,true);
      println!("{:?}",person);

      println!("{:?}",person.0);

      person.0="Smith";

      println!("{:?}",person.0);

  }

}