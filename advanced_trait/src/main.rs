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

fn main() {}