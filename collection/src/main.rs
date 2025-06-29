fn main() {
    vectors();
}

fn vectors(){
    /*  Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. */
    //basic syntax  let v: Vec<i32> = Vec::new();
    let mut vec= Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}",vec);//you can directly print like we do in other datas
    
}
