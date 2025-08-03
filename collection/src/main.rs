use std::io;

mod hashmaps;

fn main() {
    vectors();
    hashmaps::hash_maps();
}

fn vectors() {
    /*  Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. */
    //basic syntax  let v: Vec<i32> = Vec::new();
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    find_the_index_in_a_vector(&vec);
    println!("{:?}", vec); //you can directly print like we do in other datas
    let even_vec = even_from_vect(vec);
    println!("{:?}", even_vec);

    //another way of intializing vector
    let new_vec = vec![1, 2, 3, 4, 5]; //with macro
    println!("{:?}", new_vec);
}

fn even_from_vect(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            println!("{}", val);
            new_vec.push(val);
        }
    }

    return new_vec;
}

fn find_the_index_in_a_vector(vec: &Vec<i32>) {
    println!("Tell The Number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please Type A Number!");
            0
        }
    };
    let mut indx = -1;
    let mut curr_idx=0;
        for val in vec {
            curr_idx+=1;
            if *val == input {
                indx =curr_idx ;
                break;
            }
        }

        if indx!=-1 {
            println!("the item found in {}",indx)
        }else {
            println!("nothing found")
        }
}
