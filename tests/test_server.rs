use std::sync::Once;

use tools_programmer_homework::{Error, Payload};

static SPAWN_APP: Once = Once::new();

static TEST_PORT: u16 = 9998;
static URL: &str = "http://localhost:9998/";

fn spawn_app() {
    SPAWN_APP.call_once(|| {
        println!("Starting test server");
        let server = tools_programmer_homework::run(TEST_PORT);

        let _ = tokio::spawn(server);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_invalid_start() {
        spawn_app();

        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: vec![0xa9, 0xbd, 0xa0, 0xbd],
            start_address: Some(5),
            end_address: None,
        };

        let res: Error = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Error = Error {
            message: "Start address is out of bounds".to_string(),
        };

        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn test_invalid_end() {
        spawn_app();

        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: vec![0xa9, 0xbd, 0xa0, 0xbd],
            start_address: None,
            end_address: Some(5),
        };

        let res: Error = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Error = Error {
            message: "End address is out of bounds".to_string(),
        };

        assert_eq!(expected, res);
    }
}
