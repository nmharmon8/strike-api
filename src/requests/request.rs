use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::errors::{LNError, ResponseError};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Requestable {

    fn get_api_key(&self) -> &str;

    fn get_authorization_headers(&self) -> HeaderMap 
    {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.get_api_key())).unwrap(),
        );
        return headers;
    }

    fn get_url(&self) -> String;

    fn get_body(&self) -> String {
        return String::from("{}");
    }

    async fn post<T : DeserializeOwned>(&self) -> Result<T, LNError> 
    {
        let url = self.get_url();
        let body = self.get_body();
        let response = reqwest::Client::builder()
            .default_headers(self.get_authorization_headers())
            .build()?
            .post(&url)
            .body(body)
            .send()
            .await
            .map_err(|e| LNError::HTTPError(e.to_string()))?;

        //There must be a better way to do this
        match response.status() {
            reqwest::StatusCode::CREATED => {
                response.json::<T>().await.map_err(|err| {
                    LNError::JsonError(err.to_string())
                })
            },
            _ => {
                Err(LNError::HTTPResponseError(ResponseError {
                    status: response.status().as_u16(),
                    err: response.text().await.unwrap_or("".to_string()),
                    })
                )
            }
        }
    }

    async fn get<T : DeserializeOwned>(&self) -> Result<T, LNError> 
    {
        let url = self.get_url();
        let response = reqwest::Client::builder()
            .default_headers(self.get_authorization_headers())
            .build()?
            .get(&url)
            .send()
            .await
            .map_err(|e| LNError::HTTPError(e.to_string()))?;

        //There must be a better way to do this
        match response.status() {
            reqwest::StatusCode::OK => {
                response.json::<T>().await.map_err(|err| {
                    LNError::JsonError(err.to_string())
                })
            },
            _ => {
                Err(LNError::HTTPResponseError(ResponseError {
                    status: response.status().as_u16(),
                    err: response.text().await.unwrap_or("".to_string()),
                    })
                )
            }
        }
    }

    async fn delete(&self) -> Result<(), LNError> 
    {
        let url = self.get_url();
        let response = reqwest::Client::builder()
            .default_headers(self.get_authorization_headers())
            .build()?
            .delete(&url)
            .send()
            .await
            .map_err(|e| LNError::HTTPError(e.to_string()))?;

        //There must be a better way to do this
        match response.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(())
            },
            _ => {
                Err(LNError::HTTPResponseError(ResponseError {
                    status: response.status().as_u16(),
                    err: response.text().await.unwrap_or("".to_string()),
                    })
                )
            }
        }
    }

    async fn patch<T : DeserializeOwned>(&self) -> Result<T, LNError> 
    {
        let url = self.get_url();
        let body = self.get_body();
        let response = reqwest::Client::builder()
            .default_headers(self.get_authorization_headers())
            .build()?
            .patch(&url)
            .body(body)
            .send()
            .await
            .map_err(|e| LNError::HTTPError(e.to_string()))?;

        //There must be a better way to do this
        match response.status() {
            reqwest::StatusCode::OK => {
                response.json::<T>().await.map_err(|err| {
                    LNError::JsonError(err.to_string())
                })
            },
            _ => {
                Err(LNError::HTTPResponseError(ResponseError {
                    status: response.status().as_u16(),
                    err: response.text().await.unwrap_or("".to_string()),
                    })
                )
            }
        }
    }
}