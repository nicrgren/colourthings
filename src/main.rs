use anyhow::{anyhow, bail, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;

    let lock = client.request_lock().await?;
    println!("Got lock: {:?}", lock);
    let cols = Colours {
        led0: Colour::rgb(255, 0, 0),
        led1: Colour::rgb(255, 0, 0),
        led2: Colour::rgb(255, 0, 0),
        led3: Colour::rgb(255, 0, 0),
        led4: Colour::rgb(255, 0, 0),
        led5: Colour::rgb(255, 0, 0),
        led6: Colour::rgb(255, 0, 0),
        led7: Colour::rgb(255, 0, 0),
        led8: Colour::rgb(255, 0, 0),
        led9: Colour::rgb(255, 0, 0),
    };

    println!("Setting colours: {:?}", cols);

    client.set_colour(&lock, cols).await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Lock {
    hash: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Status {
    code: u64,
    description: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct LockResponse {
    hash: Option<String>,
    maxtime: Option<f64>,
    status: Status,
}

#[derive(Clone, Debug, serde::Serialize)]
struct Colour(Vec<u8>);

impl Colour {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(vec![red, green, blue])
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct Colours {
    #[serde(rename = "0")]
    led0: Colour,
    #[serde(rename = "1")]
    led1: Colour,
    #[serde(rename = "2")]
    led2: Colour,
    #[serde(rename = "3")]
    led3: Colour,
    #[serde(rename = "4")]
    led4: Colour,
    #[serde(rename = "5")]
    led5: Colour,
    #[serde(rename = "6")]
    led6: Colour,
    #[serde(rename = "7")]
    led7: Colour,
    #[serde(rename = "8")]
    led8: Colour,
    #[serde(rename = "9")]
    led9: Colour,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SetColourRequest {
    hash: String,
    colours: String,
}

struct Client {
    client: reqwest::Client,
}

impl Client {
    fn new() -> Result<Self> {
        let client = reqwest::ClientBuilder::new().build()?;

        Ok(Self { client })
    }

    async fn set_colour(&self, lock: &Lock, cols: Colours) -> Result<()> {
        let colorsjson = serde_json::to_string(&cols).expect("Serializing colors");

        let req = SetColourRequest {
            hash: lock.hash.clone(),
            colours: colorsjson,
        };

        let res = self
            .client
            .get("http://api.colourbynumbers.org/cbn-live/setColours")
            .header("Accept", "application/json")
            .header("Accept-Language", "sv-se")
            .header("Connection", "keep-alive")
            .query(&req)
            .send()
            .await?;

        let body = res.text().await?;

        println!("Got response: {}", body);
        Ok(())
    }

    async fn request_lock(&self) -> Result<Lock> {
        let res = self
            .client
            .get("http://api.colourbynumbers.org/cbn-live/requestLock")
            .header("Accept", "application/json")
            .header("Accept-Language", "sv-se")
            .header("Connection", "keep-alive")
            .send()
            .await?;

        let bs = res.bytes().await?;

        let res = serde_json::from_slice::<LockResponse>(&bs).map_err(|err| {
            anyhow!(
                "Failed to deserialize response:\n{}\n as json: {}",
                String::from_utf8_lossy(&bs),
                err
            )
        })?;

        match res.status.code {
            0 => Err(anyhow!("Busy")),
            1 => Ok(Lock {
                hash: res.hash.ok_or_else(|| anyhow!("Hash missing from resp"))?,
            }),
            n => {
                bail!(
                    "Unknown status `{}` with body: {}",
                    n,
                    String::from_utf8_lossy(&bs)
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn colour_serialize() {
        let col = Colour::rgb(128, 127, 129);

        assert_eq!(r#"[128,127,129]"#, serde_json::to_string(&col).unwrap())
    }

    #[test]
    fn colours_serialize() {
        let cols = Colours {
            led0: Colour::rgb(255, 0, 0),
            led1: Colour::rgb(255, 0, 0),
            led2: Colour::rgb(255, 0, 0),
            led3: Colour::rgb(255, 0, 0),
            led4: Colour::rgb(255, 0, 0),
            led5: Colour::rgb(255, 0, 0),
            led6: Colour::rgb(255, 0, 0),
            led7: Colour::rgb(255, 0, 0),
            led8: Colour::rgb(255, 0, 0),
            led9: Colour::rgb(255, 0, 0),
        };

        let s = serde_urlencoded::to_string(&cols).expect("Urlencoding");
        assert_eq!(s, "bas");
    }
}
