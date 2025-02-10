fn main() {
    /* let */

    //let are block-scoped, meaning they are only accessible within the block they are
    //by default, variables are immutable in rust
    let mut x = 5; // if we assign like let x=5 then you can reassign 6 to x next time
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* const */
    const THREE_HOUR: u32 = 60 * 3; //const variables are always immutable
                                    //Constants are globally scoped and can be accessed from anywhere in the program.
                                    //for const the naming convention is UPPER_CASE (uppercase with underscore for seperation)
    println!("Three Hours: {}", THREE_HOUR);

    fn shadowing() {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;//hre value of x will be 12
            println!("The value of x in the inner scope is: {x}");

            //after ending this scope the x value will comes to 6 again
        }

        println!("The value of x is: {x}");
    }

    shadowing();
}
