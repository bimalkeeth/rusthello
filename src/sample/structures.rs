
#[derive(Debug)]
struct Employee{
    name: String,
    company: String,
    age:i32
}

impl Employee {
    fn struct_details(&self)->String{
        format!("name:{},age:{},company:{}",
                &self.name,&self.company,&self.age)
    }
}


pub fn show_struct(){
   let emp = Employee{
       age:45,
       name:String::from("Bimal"),
       company :String::from("WSD")
   };

   println!("{}", emp.struct_details());
}