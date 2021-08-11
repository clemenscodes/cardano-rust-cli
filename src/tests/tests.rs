#[cfg(test)]
pub mod test {
    use crate::*;
    use crate::utils::Terminal;
    use console::Emoji;

    #[tokio::test]
    async fn async_command() {
        let res = Terminal::async_command("white", "cardano-node --version", Emoji("", "")).await;
        assert!(res.is_ok());
    }
}