/*
    ! CLI NOTES
    * `cargo test -- --test-threads=1`
        ? will not run in parallel
    * `cargo test -- --show-output`
        ? will print println! in cli
    * `cargo test {name}`
        ? name of function will only run that function
        ? example run_this_only* will
        ! but, the way it works is, it runs a catchall with name so running it_ will run it_works and it_doesnt_work
    * `cargo test -- --ignored`
        ? Will run ignored tests.

*/
#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::prints_and_returns_10;
    #[test]
    fn it_works() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    // ! A fail means its a pass!
    #[should_panic]
    fn it_doesnt_work() {
        let value = prints_and_returns_10(4);
        assert_eq!(5, value);
    }
    #[test]
    fn run_this_only() {
        println!("run_this_only ran");
        assert_eq!(5, 5);
    }
    #[test]
    // ! Ignore this function by default
    #[ignore]
    fn ignored() {
        assert_eq!(5, 10); // this function will be ignored, fail or pass wont matter
    }
}
