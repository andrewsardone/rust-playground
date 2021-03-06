// http://www.rustforrubyists.com/book/chapter-05.html
// Tinkering around with FizzBuzz

use std::io::println;

fn is_three(num: int) -> bool {
    num % 3 == 0
}

fn is_five(num: int) -> bool {
    num % 5 == 0
}

fn fizzbuzz(num: int) -> ~str {
    match (is_three(num), is_five(num)) {
        (true, true) => ~"FizzBuzz",
        (true, false) => ~"Fizz",
        (false, true) => ~"Buzz",
        (false, false) => num.to_str(),
    }
}

fn main() {
    for num in range(1,101) {
        println(fizzbuzz(num));
    }
}

#[test]
fn test_is_three_with_not_three() {
    assert!(!is_three(1))
}

#[test]
fn test_is_three_with_three() {
    assert!(is_three(3))
}

#[test]
fn test_is_five_with_not_five() {
    assert!(!is_five(1))
}

#[test]
fn test_is_five_with_five() {
    assert!(is_five(5))
}

#[test]
fn test_fizbuzz_prints_number() {
    assert!(fizzbuzz(1) == ~"1");
    assert!(fizzbuzz(2) == ~"2");
}

#[test]
fn test_fizzbuzz_prints_fizz() {
    assert!(fizzbuzz(3) == ~"Fizz");
    assert!(fizzbuzz(9) == ~"Fizz");
}

#[test]
fn test_fizzbuzz_prints_buzz() {
    assert!(fizzbuzz(5) == ~"Buzz")
    assert!(fizzbuzz(10) == ~"Buzz")
}

#[test]
fn test_fizzbuzz_prints_fizzbuzz() {
    assert!(fizzbuzz(15) == ~"FizzBuzz");
    assert!(fizzbuzz(30) == ~"FizzBuzz");
}
