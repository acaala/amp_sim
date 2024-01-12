use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    str::{Bytes, FromStr},
};

use anyhow::Error;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct Assistant {
    pub api_key: Option<String>,
    pub thread_id: Option<String>,
}

impl Assistant {
    pub fn new() -> Self {
        Self {
            api_key: None,
            thread_id: None,
        }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let auth_string = "Bearer ".to_owned() + self.api_key.as_ref().unwrap();

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth_string).unwrap(),
        );
        headers.insert("OpenAI-Beta", HeaderValue::from_static("assistants=v1"));

        headers
    }

    pub async fn init_thread(&mut self) -> Result<String, Error> {
        let url = "https://api.openai.com/v1/threads";

        let headers = self.build_headers();

        let response = Client::new()
            .post(url)
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("{:?}", response["id"]);

        let thread_id = response["id"].as_str().unwrap().to_owned();

        self.thread_id = Some(thread_id.clone());

        Ok(thread_id)
    }

    pub async fn retrieve_messages(&self) -> Result<Value, Error> {
        let url = "https://api.openai.com/v1/threads/".to_owned()
            + self.thread_id.as_ref().unwrap()
            + "/messages";

        let headers = self.build_headers();

        let response = Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("/n Retrieved message");
        println!("{:?}", response);

        Ok(response)
    }

    pub async fn send_message(&self, prompt: String) -> Result<(), Error> {
        let url = "https://api.openai.com/v1/threads/".to_owned()
            + self.thread_id.as_ref().unwrap()
            + "/messages";

        let headers = self.build_headers();

        let json_data = json!({
            "role": "user",
            "content": prompt
        });

        let response = Client::new()
            .post(url)
            .headers(headers)
            .json(&json_data)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("{:?}", response);

        Ok(())
    }

    pub async fn run_assistant(&self) -> Result<String, Error> {
        let url = "https://api.openai.com/v1/threads/".to_owned()
            + self.thread_id.as_ref().unwrap()
            + "/runs";

        let headers = self.build_headers();

        let json_data = json!({
            "assistant_id": "asst_DXW9s2TeaZGXxwfc85rC5v6v",
        });

        println!("Sending");

        let response = Client::new()
            .post(url)
            .headers(headers)
            .json(&json_data)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("Runs");
        println!("{:?}", response);

        Ok(response["id"].as_str().unwrap().to_owned())
    }

    pub async fn check_run_status(&self, run_id: String) -> Result<String, Error> {
        let url = "https://api.openai.com/v1/threads/".to_owned()
            + self.thread_id.as_ref().unwrap()
            + "/runs/"
            + &run_id;

        let headers = self.build_headers();

        let response = Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("{:?}", response["status"]);

        Ok(response["status"].as_str().unwrap().to_owned())
    }
}

//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
