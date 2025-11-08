use anyhow::Result;
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use std::collections::HashMap;

pub struct HttpClient {
    pub base_url: String,
    pub api_key: Option<String>,
    pub client: Client,
}

impl HttpClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url,
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get(
        &self,
        path: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let request = self._build_request(Method::GET, path, headers, params, None)?;
        let response = request.send().await?;
        Ok(response)
    }

    pub async fn post(
        &self,
        path: &str,
        body: Option<HashMap<String, String>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let request = self._build_request(Method::POST, path, headers, None, body)?;
        let response = request.send().await?;
        Ok(response)
    }

    pub async fn patch(
        &self,
        path: &str,
        body: Option<HashMap<String, String>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let request = self._build_request(Method::PATCH, path, headers, None, body)?;
        let response = request.send().await?;
        Ok(response)
    }

    pub async fn delete(
        &self,
        path: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Response> {
        let request = self._build_request(Method::DELETE, path, headers, None, None)?;
        let response = request.send().await?;
        Ok(response)
    }

    fn _build_request(
        &self,
        method: Method,
        path: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
        body: Option<HashMap<String, String>>,
    ) -> Result<RequestBuilder> {
        let url = self._build_url(path)?;
        let mut request = self.client.request(method, url);
        request = request
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        if let Some(params) = params {
            request = request.query(&params);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        Ok(request)
    }

    fn _build_url(&self, path: &str) -> Result<Url> {
        if path.starts_with("/") {
            Ok(Url::parse(&format!("{}{}", self.base_url, path))?)
        } else {
            Ok(Url::parse(&format!("{}/{}", self.base_url, path))?)
        }
    }
}
