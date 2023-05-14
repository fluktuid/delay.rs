

pub async fn check(name: &str, ready_timeout: Duration) -> Result<()> {
    match TcpStream::connect(&backend_address).await {
        Ok(_) => OK()
        ERR(_) => ERR()
    }
}