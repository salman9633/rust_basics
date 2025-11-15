fn main() {
    /* 1 */
    normal_loop_iterator();

    /* 2  :  iterating after creatin an iterator : .iter()*/
    iterating_after_iterator();

    /* 3  :  mutable iterator : .iter_mut()*/
    mutable_iterator();

    /* 4  : iterate using .next : .next() */
    using_next();
}

fn normal_loop_iterator() {
    let nums = vec![1, 2, 3];

    for val in nums {
        println!("{}", val);
    }
}

fn iterating_after_iterator() {
    let nums = vec![1, 2, 3];
    let iter = nums.iter();

    for val in iter {
        // val = val * 2;  we can't do this bcoz the val is immutable as it will give u an error
        let a = val * 2;
        println!("{}", a);
    }
    println!("{:?}", nums); //we can use nums's data even iter variable bcoz we are saving the referece in iter but wont change the intial values

    /*
    The iter() method in Rust providesa way to iterate over an elements of a collection borrowing them

    we can't mutate the variables since we have an immutable reference to the internal elements
    */
}

fn mutable_iterator() {
    let mut nums = vec![1, 2, 3];
    let iter = nums.iter_mut();

    for val in iter {
        *val = *val * 2;
        println!("{}", val);
    }
    println!("{:?}", nums);//o/p will be [2, 4, 6]
}

fn using_next(){
    let mut nums = vec![1,2,3];

    let mut iter = nums.iter();

    while let Some(val)= iter.next(){
        println!("{}",val)
    }
}