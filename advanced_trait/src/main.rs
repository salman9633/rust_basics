/*
Associate type

Generic + Trait
*/

trait Iterator<T>{
 fn next(&mut self)->Option<T>;
}

struct Counter{}

// As We used Generics as type in Iterator we can pass different data types while implementing
impl Iterator<u32>for Counter{
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator<u16> for Counter{
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

/*
using Same Function name
*/

struct Human;

trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

impl Human{
    fn fly(&self){
        println!("Log from Struct Impl")
    }
}

impl Pilot for Human{
    fn fly(&self) {
        println!("Log From Pilot Impl")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Log From Wizard impl")
    }
}


fn main() {
    let person=Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}