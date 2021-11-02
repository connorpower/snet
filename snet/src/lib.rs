pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_foo() {
        assert_eq!(1 + 2, 3);
    }

}
