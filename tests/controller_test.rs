use chrono::{DateTime, Utc};
use httpc_test::Response;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub _id: String,
    pub email: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHandler<T = UserData> {
    pub data: Option<T>,
    pub code: u16,
    pub message: Option<String>,
    pub status: String,
    pub date: DateTime<Utc>,
}

#[tokio::test]
async fn test_create_user() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let body = json!({
        "email": "here--id@gmail.com",
        "first_name": "silav",
        "password": "test-password",

    });

    let resp: Response = client.do_post("/users/create", body).await?;
    resp.print().await?;

    let json_value: serde_json::Value = resp.json_body()?;
    let response_body: ResponseHandler<UserData> = from_value(json_value)?;

    if let Some(user_data) = response_body.data {
        let url = format!("/users/{}", user_data._id);
        let resp: Response = client.do_get(url.as_str()).await?;
        resp.print().await?;

        assert_eq!(user_data._id, "what@gmail.com");
    } else {
        assert!(false);
    }

    Ok(())
}
#[tokio::test]
async fn test_update_user() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let body = json!({
        "email": "updated@gmail.com",
        "first_name": "updated: silav",
        "password": "updated: test-password",

    });

    let expected_user_id = "678398f04d4e14a53e64a25d";
    let url = format!("/users/{}", expected_user_id);

    let resp: Response = client.do_put(url.as_str(), body).await?;
    resp.print().await?;

    let json_value: serde_json::Value = resp.json_body()?;
    let response_body: ResponseHandler<UserData> = from_value(json_value)?;

    if let Some(user_data) = response_body.data {
        assert_eq!(user_data._id, "what@gmail.com");
    } else {
        assert!(false);
    }

    Ok(())
}

#[tokio::test]
async fn test_get_user() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let expected_user_id = "678398f04d4e14a53e64a25d";
    let url = format!("/users/{}", expected_user_id);
    let resp: Response = client.do_get(url.as_str()).await?;
    resp.print().await?;

    let json_value: serde_json::Value = resp.json_body()?;
    let response_body: ResponseHandler<UserData> = from_value(json_value)?;

    if let Some(user_data) = response_body.data {
        assert_eq!(user_data._id, expected_user_id);
    } else {
        assert!(false);
    }

    Ok(())
}

#[tokio::test]
async fn test_delete_user() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let expected_user_id = "678398cc4d4e14a53e64a25b";
    let url = format!("/users/{}", expected_user_id);
    let resp: Response = client.do_delete(url.as_str()).await?;
    resp.print().await?;

    let body = resp.json_body()?;
    assert_eq!(resp.status(), 200);

    Ok(())
}
