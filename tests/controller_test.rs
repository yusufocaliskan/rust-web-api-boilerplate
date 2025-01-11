use httpc_test::Response;
use serde_json::json;

#[tokio::test]
async fn user_controller() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let body = json!({

        "email": "new-email@bar.com",

        "first_name": "foo foo",
        "password": "test-password",

    });
    let resp: Response = client.do_post("/users/create", body).await?;
    resp.print().await.unwrap();

    let body = resp.json_body()?;
    assert_eq!(resp.status(), 201);

    // assert_eq!(body["email"], "silav@bar.com");

    Ok(())
}