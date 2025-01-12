use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct TodoData {
    id: String,
    title: String,
    description: String,
    completed: bool,
    user_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct Message {
    message: String,
    status_code: u16,
}

#[derive(Debug, Deserialize)]
struct ResponseHandler<T = TodoData> {
    message: Message,
    data: Option<T>,
    status: bool,
    date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct TodoListResponse {
    message: Message,
    data: Option<Vec<TodoData>>,
    status: bool,
    date: DateTime<Utc>,
}

#[tokio::test]
async fn test_create_todo() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    // Create user first
    let user_id = create_test_user(&client, base_url).await?;

    let todo_body = json!({
        "title": "Test Todo Creation",
        "description": "Testing todo creation functionality",
        "user_id": user_id
    });

    let response = client
        .post(&format!("{}/todos", base_url))
        .json(&todo_body)
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 201);
    let body: ResponseHandler = response.json().await?;
    assert!(body.data.is_some());
    assert_eq!(body.data.unwrap().title, "Test Todo Creation");

    Ok(())
}

#[tokio::test]
async fn test_get_todo() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    // Create user and todo first
    let (user_id, todo_id) = create_test_user_and_todo(&client, base_url).await?;

    let response = client
        .get(&format!("{}/todos/{}", base_url, todo_id))
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 200);
    let body: ResponseHandler = response.json().await?;
    assert!(body.data.is_some());
    assert_eq!(body.data.unwrap().user_id, user_id);

    Ok(())
}

#[tokio::test]
async fn test_update_todo() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    let (_, todo_id) = create_test_user_and_todo(&client, base_url).await?;

    let update_body = json!({
        "title": "Updated Todo Title",
        "description": "Updated description",
        "completed": true
    });

    let response = client
        .put(&format!("{}/todos/{}", base_url, todo_id))
        .json(&update_body)
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 200);
    let body: ResponseHandler = response.json().await?;
    assert!(body.data.is_some());
    let updated_todo = body.data.unwrap();
    assert_eq!(updated_todo.title, "Updated Todo Title");
    assert_eq!(updated_todo.completed, true);

    Ok(())
}

#[tokio::test]
async fn test_delete_todo() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    let (_, todo_id) = create_test_user_and_todo(&client, base_url).await?;

    let response = client
        .delete(&format!("{}/todos/{}", base_url, todo_id))
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 200);

    // Verify deletion
    let verify_response = client
        .get(&format!("{}/todos/{}", base_url, todo_id))
        .send()
        .await?;

    assert_eq!(verify_response.status().as_u16(), 404);

    Ok(())
}

#[tokio::test]
async fn test_get_all_todos() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    // Create multiple todos
    let (user_id, _) = create_test_user_and_todo(&client, base_url).await?;
    create_additional_todo(&client, base_url, &user_id).await?;

    let response = client.get(&format!("{}/todos", base_url)).send().await?;

    assert_eq!(response.status().as_u16(), 200);
    let body: TodoListResponse = response.json().await?;
    assert!(body.data.is_some());
    assert!(body.data.unwrap().len() >= 2);

    Ok(())
}

#[tokio::test]
async fn test_get_user_todos() -> anyhow::Result<()> {
    let client = Client::new();
    let base_url = "http://localhost:4040/api/v1";

    let (user_id, _) = create_test_user_and_todo(&client, base_url).await?;
    create_additional_todo(&client, base_url, &user_id).await?;

    let response = client
        .get(&format!("{}/todos/user/{}", base_url, user_id))
        .send()
        .await?;

    assert_eq!(response.status().as_u16(), 200);
    let body: TodoListResponse = response.json().await?;
    assert!(body.data.is_some());
    let todos = body.data.unwrap();
    assert!(todos.len() >= 2);
    assert!(todos.iter().all(|todo| todo.user_id == user_id));

    Ok(())
}

// Helper Functions
async fn create_test_user(client: &Client, base_url: &str) -> anyhow::Result<String> {
    let user_body = json!({
        "email": "heeee@example.com",
        "first_name": "Todo",
        "password": "test-password",
    });

    let response = client
        .post(&format!("{}/users/create", base_url))
        .json(&user_body)
        .send()
        .await?;

    let user_data: ResponseHandler = response.json().await?;
    Ok(user_data.data.unwrap().id)
}

async fn create_test_user_and_todo(
    client: &Client,
    base_url: &str,
) -> anyhow::Result<(String, String)> {
    let user_id = create_test_user(client, base_url).await?;

    let todo_body = json!({
        "title": "Test Todo",
        "description": "This is a test todo",
        "user_id": user_id
    });

    let response = client
        .post(&format!("{}/todos", base_url))
        .json(&todo_body)
        .send()
        .await?;

    let todo_data: ResponseHandler = response.json().await?;
    Ok((user_id, todo_data.data.unwrap().id))
}

async fn create_additional_todo(
    client: &Client,
    base_url: &str,
    user_id: &str,
) -> anyhow::Result<String> {
    let todo_body = json!({
        "title": "Additional Todo",
        "description": "This is an additional test todo",
        "user_id": user_id
    });

    let response = client
        .post(&format!("{}/todos", base_url))
        .json(&todo_body)
        .send()
        .await?;

    let todo_data: ResponseHandler = response.json().await?;
    Ok(todo_data.data.unwrap().id)
}
