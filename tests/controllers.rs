// #[cfg(test)]
//
// mod lesson_controller_tests {
//     use actix_web::{test, web, App};
//
//     use async_trait::async_trait;
//     use shaku::{module, Component};
//
//     // Mock for ILessonsService
//     #[async_trait]
//     pub trait ILessonsServiceMock: ILessonsService {
//         async fn lessons(&self);
//     }
//
//     #[derive(Component)]
//     #[shaku(interface = ILessonsService)]
//     struct LessonsServiceMock;
//
//     #[async_trait]
//     impl ILessonsService for LessonsServiceMock {
//         async fn lessons(&self) {
//             // Mock implementation
//             println!("Mock lessons service called");
//         }
//     }
//
//     // Mock for IDatabase
//     #[derive(Component)]
//     #[shaku(interface = IDatabase)]
//     struct DatabaseMock;
//
//     impl IDatabase for DatabaseMock {
//         fn get_db(&self) -> mongodb::Database {
//             // Mock database instance or No-op
//             todo!()
//         }
//     }
//
//     #[actix_web::test]
//     async fn test_find_all() {
//         // Create a mock AppModules module
//         module! {
//             AppModulesMock {
//                 components = [LessonsServiceMock, DatabaseMock],
//                 providers = []
//             }
//         }
//
//         let module = AppModulesMock::builder().build();
//
//         // Initialize AppState and inject dependencies
//         let app_state = AppState {
//             // Fill in the necessary fields for your AppState
//         };
//
//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(app_state))
//                 .app_data(web::Data::new(module))
//                 .route("/lessons", web::get().to(LessonController::find_all)),
//         )
//         .await;
//
//         // Send a test request
//         let req = test::TestRequest::get().uri("/lessons").to_request();
//
//         let resp = test::call_service(&app, req).await;
//
//         // Assert the response
//         assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
//
//         let body = test::read_body(resp).await;
//         assert_eq!(body, "Hello::find-all");
//     }
// }
