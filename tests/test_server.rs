use tools_programmer_homework::{Error, Payload};

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_invalid_start() {
        const URL: &str = "http://localhost:9999/";
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
        const URL: &str = "http://localhost:9999/";
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
