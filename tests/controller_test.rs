use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub _id: String,
    pub first_name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHandler<T = UserData> {
    pub data: Option<T>,
    pub code: u16,
    pub message: Option<String>,
    pub status: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedInUser {
    pub access_token: String,
    pub user: UserData,
}

pub async fn ensure_logged_in_user() -> anyhow::Result<LoggedInUser> {
    let body = json!({
        "email": "t43434here--id@gmail.com",
        "password": "test-password",
    });

    let client = Client::new();
    let resp = client
        .post("http://localhost:4040/api/v1/users/login")
        .json(&body)
        .send()
        .await?;

    println!("Login Response --> {:#?}", resp);
    let response_text = resp.text().await?;
    println!("Login Body --> {:#?}", response_text);
    let response_body: ResponseHandler<LoggedInUser> = serde_json::from_str(&response_text)?;

    if let Some(user) = response_body.data {
        return Ok(user);
    }

    anyhow::bail!("Failed to log in");
}

#[tokio::test]
async fn test_create_user() -> anyhow::Result<()> {
    let client = Client::new();

    let body = json!({
        "email": "t43434here--id@gmail.com",
        "first_name": "silav",
        "password": "test-password",
    });

    let resp = client
        .post("http://localhost:4040/api/v1/users/create")
        .json(&body)
        .send()
        .await?;

    println!("Create User Response --> {:#?}", resp);
    let response_text = resp.text().await?;
    let response_body: ResponseHandler<UserData> = serde_json::from_str(&response_text)?;

    if let Some(user_data) = response_body.data {
        assert_eq!(user_data.email, "t43434here--id@gmail.com");
    } else {
        anyhow::bail!("User creation failed");
    }

    Ok(())
}

#[tokio::test]
async fn test_login_user() -> anyhow::Result<()> {
    let user = ensure_logged_in_user().await?;
    println!("Logged in user: {:#?}", user);

    assert_eq!(user.user.email, "id@gmail.com");
    Ok(())
}

#[tokio::test]
async fn test_update_user() -> anyhow::Result<()> {
    let client = Client::new();
    let user = ensure_logged_in_user().await?;

    println!("Logged in user: {:#?}", user);
    let body = json!({
        "email": "t43434here--id@gmail.com",
        "first_name": "updated: silav",
        "password": "updated: test-password",
    });

    let expected_user_id = user.user._id;
    println!("ID--> Updated User --> {:#?}", expected_user_id);
    let url = format!("http://localhost:4040/api/v1/users/{}", expected_user_id);

    let resp = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", user.access_token))
        .json(&body)
        .send()
        .await?;

    println!("Update User Response --> {:#?}", resp);
    let response_body: ResponseHandler<UserData> = resp.json().await?;

    if let Some(updated_user) = response_body.data {
        assert_eq!(updated_user.email, "updated@gmail.com");
    } else {
        anyhow::bail!("User update failed");
    }

    Ok(())
}

#[tokio::test]
async fn test_delete_user() -> anyhow::Result<()> {
    let client = Client::new();
    let user = ensure_logged_in_user().await?;

    let expected_user_id = user.user._id;
    let url = format!("http://localhost:4040/api/v1/users/{}", expected_user_id);

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", user.access_token))
        .send()
        .await?;

    println!("Delete User Response --> {:#?}", resp);
    assert_eq!(resp.status().as_u16(), 200);

    Ok(())
}
