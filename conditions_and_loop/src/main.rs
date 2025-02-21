fn main() {
    let is_even:bool = false;

    if is_even{//no need of brackets
        println!("The number is even");
    }else if !is_even{
        println!("The number is odd");
    }

    //loops

    for i in 0..10{// it will loop from 0 to 9
        println!("The value of i is: {i}");
    }

    let sentence= String::from("My Name is Salman");

    let first_word=find_the_first_word(sentence);
    println!("The first word is: {first_word}");
}

fn find_the_first_word(sentence:String)->String//mentioning the return type
{

    let mut ans=String::new();

    for chars in sentence.chars(){
        ans.push_str(chars.to_string().as_str());
        if chars==' '{
            break;
        }
    }
    return ans;
}
