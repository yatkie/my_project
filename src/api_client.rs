use serde::Deserialize;
use std::error::Error;

const BASE_URL: &str = "http://127.0.0.1:8080";

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub variant: u32,
    pub watermark: String,
}

#[derive(Debug, Deserialize)]
pub struct Reader {
    pub variant: u32,
    pub client_id: String,
    pub reader_id: String,
    pub reader_name: String,
    pub library_card: String,
    pub watermark: String,
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub variant: u32,
    pub total_books: u32,
    pub available_books: u32,
    pub active_loans: u32,
    pub watermark: String,
}

#[derive(Debug, Deserialize)]
pub struct BooksResponse {
    pub variant: u32,
    pub count: u32,
    pub watermark: String,
}

#[derive(Debug, Deserialize)]
pub struct LoansResponse {
    pub variant: u32,
    pub count: u32,
    pub watermark: String,
}

pub struct ApiClient {
    client: reqwest::blocking::Client,
    token: String,
}

impl ApiClient {
    pub fn authenticate(client_id: &str, client_secret: &str) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(format!("{BASE_URL}/auth/token"))
            .json(&serde_json::json!({
                "client_id": client_id,
                "client_secret": client_secret
            }))
            .send()?
            .error_for_status()?;

        let token: TokenResponse = response.json()?;
        println!("Token expires in: {} seconds", token.expires_in);
        println!("Server variant: {}", token.variant);
        println!("Server watermark: {}", token.watermark);

        if token.token_type.to_lowercase() != "bearer" {
            return Err("Unsupported token type".into());
        }

        Ok(Self {
            client,
            token: token.access_token,
        })
    }

    pub fn get_reader(&self) -> Result<Reader, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{BASE_URL}/whoami"))
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?;

        Ok(response.json()?)
    }

    pub fn get_books(&self) -> Result<BooksResponse, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{BASE_URL}/api/books"))
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?;

        Ok(response.json()?)
    }

    pub fn get_loans(&self) -> Result<LoansResponse, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{BASE_URL}/api/loans"))
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?;

        Ok(response.json()?)
    }

    pub fn get_summary(&self) -> Result<Summary, Box<dyn Error>> {
        let response = self
            .client
            .get(format!("{BASE_URL}/api/summary"))
            .bearer_auth(&self.token)
            .send()?
            .error_for_status()?;

        Ok(response.json()?)
    }
}
