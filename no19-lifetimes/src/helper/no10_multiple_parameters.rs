#![allow(unused)]

// fn choose_favorite<'a>(first: &str, second: &'a str) -> &'a str {
//     println!("{first}");
//     second
// }

fn choose_favorite<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("{first}");
    println!("{second}");
    first
    // second // if we return second, Error: lifetime may not live long enough. (due to conflict with general lifetime annotation)
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    // println!("The second is {second}");
    // first
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn longest_multiple<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("The second is {second}");
    first
}

pub fn main() {
    println!("=== no10_multiple_parameters ===");
    let orlando = String::from("Orlando");
    let san_francisco = String::from("San Francisco");
    let result = longest(&orlando, &san_francisco);
    println!("{result}");

    let orlando = String::from("Orlando");
    {
        let san_francisco = String::from("San Francisco");
        let result = longest(&orlando, &san_francisco);
        println!("{result}");
    }

    // compile error example
    // let orlando = String::from("Orlando");
    // let result = {
    //     let san_francisco = String::from("San Francisco");
    //     longest(&orlando, &san_francisco) // san_francisco lifetime end here.
    // };

    let orlando = String::from("Orlando");
    let result = {
        let san_francisco = String::from("San Francisco");
        longest_multiple(&orlando, &san_francisco) // san_francisco lifetime end here. code will not compile
    };
}
