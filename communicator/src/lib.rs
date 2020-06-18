pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client; //親階層からuseで取り込む
    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        //use super::clientで取り込んでいるので以下のコードが実行できる
        client::connect();
    }
}
