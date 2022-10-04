fn main() {
    // if expression must be a bool. no truthy/falsey here... I'm looking at you javascript. 
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // else if. it is mentioned that if you have more than one if else to refactor lol. see: match;
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 }; // types in condition must match

    println!("The value of number is: {number}");

    // loops
    loop {
        println!("forever")
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // break and continue apply to the innermost loop
      let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // break from specified loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    
}

