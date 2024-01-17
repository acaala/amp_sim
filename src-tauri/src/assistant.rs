use std::collections::HashMap;

use anyhow::Error;
use regex::Regex;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
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

        let response = Client::new()
            .post(url)
            .headers(headers)
            .json(&json_data)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("Running Assistant");

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

        println!("Run Status: {:?}", response["status"]);

        Ok(response["status"].as_str().unwrap().to_owned())
    }

    pub fn get_parsed_assistant_response(
        &self,
        initial_response: Value,
    ) -> Result<AssistantResponse, Error> {
        let response_str = &initial_response["data"][0]["content"][0]["text"]["value"].to_string();

        let re = Regex::new(r"\\n|\\|```json|```")?;
        let mut cleaned_response = re.replace_all(response_str, "").to_string();

        cleaned_response = cleaned_response.trim_matches('"').to_string();

        let res: AssistantResponse = serde_json::from_str(&cleaned_response)?;

        println!("Assistant Response: {:?}", res);

        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssistantResponse {
    pub processors: Vec<HashMap<String, HashMap<String, f32>>>,
}
