// http://www.rustforrubyists.com/book/chapter-04.html
// An example of using Rust's built-in test annotation — seemingly no
// arguments, returns, or naming convention needed.
//
// Compile with the --test flag (test harness with special main function):
//
//     rustc --test testing.rs

#[test]
fn this_tests_code() {
    fail!("Fail!");
}
