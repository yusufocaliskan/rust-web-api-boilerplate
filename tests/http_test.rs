use httpc_test::Response;
use serde_json::json;

#[tokio::test]
async fn http_test() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://localhost:4040/api/v1")?;

    let body = json!({
        "email": "no@bar.com",

        "first_name": "foo foo",
        "password": "test-password",
        "user_id":2121

    });
    let resp: Response = client.do_post("/users/create", body).await?;
    resp.print().await.unwrap();

    let body = resp.json_body()?;
    assert_eq!(resp.status(), 200);
    // assert_eq!(body["email"], "silav@bar.com");

    Ok(())
}
