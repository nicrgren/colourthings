use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;

    client.request_lock().await?;

    Ok(())
}

enum Lock {
    Locked,
    Unlocked,
}

struct Client {
    client: reqwest::Client,
}

impl Client {
    fn new() -> Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .user_agent("CBN/4.2.5 CFNetwork/1")
            .build()?;

        Ok(Self { client })
    }

    async fn request_lock(&self) -> Result<Lock> {
        let res = self
            .client
            .get("http://api.colourbynumbers.org/cbn-live/requestLock")
            .header("Accept", "*/*")
            .header("Accept-Language", "sv-se")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Connection", "keep-alive")
            .send()
            .await?;

        let s = res.text().await?;

        println!("Got reply:\n{}", s);

        Ok(Lock::Unlocked)
    }
}
