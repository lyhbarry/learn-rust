fn main() {
    // if
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // else if
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
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while - 1
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // while - 2
    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for - iter
    let a = [10, 20, 30, 40 ,50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for - reverse
    for number in (1..4).rev() {
        println!("{}", number)
    }

    println!("LIFTOFF!!!");

    println!("\nConvert fahrenheit to celsius:");
    println!("{} celsius", fahrenheit_to_celsius(100 as f32));

    println!("\nFibonacci numbers:");
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(20));

    println!("\nChristmas carol:");
    christmas_carol();
}

fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0)  * (5.0 / 9.0)
}

fn fibonacci(n: i32) -> i32 {
    let mut n1 = 1;
    let mut n2 = 1;
    let mut result = 0;

    if n != 1 || n != 2 {
        let end = n - 2;

        for _i in 0..end {
            result = n1 + n2;
            n1 = n2;
            n2 = result;
        }
    }
    result
}

fn christmas_carol() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings, badam-pam-pam", "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for index in 0..12 {
        println!("On the {} day of Christmas\nMy true love gave to me", days[index]);
        let toReceive = &gifts[0..index+1];
        for r in toReceive.iter().rev() {
            println!("{}", r);
        }
        println!("\n");
    }
}
