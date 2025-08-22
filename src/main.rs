// jargon 3 - ownership

fn main(){
       let s1 = String::from("Hi there");

       print!("{}" , s1); //we can print here as Rihana can have fun with her boyfriend before moving on to the next

       let s2 = s1;

       //    print!("{}" , s1);
       //we cannot use s1 anymore as it is already passed to s2
       //Keep in mind Rihana can have only one boyfriend

        print!("{}" , s2);
}
//whenever owner gets out of scope heap memory gets deallocated

       