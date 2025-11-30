/*

Use tuples when:==>

You want to return multiple values from a function

You want a quick grouping of mixed values

You don't need named fields (temporary data)

Use structs when:==>

Fields need names

The data is important

You want methods via impl

*/

/* A tuple is a fixed-size group of values that can each have different types.  */

pub fn tuple_example() {
    println!("<============Tuples Starts Here ============>");

    /* Basic Tuple & printing & destructuring  */
    let user = ("SALMAN", 25, 160.5);

    println!("{} User Name", user.0);
    println!("{} User Age", user.1);
    println!("{} User Height", user.2);

    let (name, age, height) = user;

    println!("{}-{}-{}", name, age, height);

    /* Nested Tuple */

    let book = ("Dunken & Eggs", (200, 7.5));

    println!("{}", book.0); //"Dunken & Eggs"
    println!("{:?}", book.1); //(200, 7.5)

    let (name, (pages, rating)) = book;

    println!("{}-{}-{}", name, pages, rating);

    /* Changing values in tuple */

    let mut dimention = (30, 60);

    println!("{:?}", dimention);

    dimention.0 = 50;

    println!("{:?}", dimention);

    /* Function + tuple */

    let (num1, num2) = (35, 30);
    fn stat(num1: i32, num2: i32) -> (i32, i32, i32) {
        let prod = num1 * num2;
        let add = num1 + num2;
        let div = num1 / num2;

        return (prod, add, div);
    }

    let res = stat(num1, num2);

    println!("{:?}", res);
}
