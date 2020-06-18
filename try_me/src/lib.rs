pub mod outeromost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}
fn try_me() {
    outeromost::middle_function();
    outeromost::middle_secret_function();
    outeromost::inside::inner_function();
    outeromost::inside::secret_function();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
