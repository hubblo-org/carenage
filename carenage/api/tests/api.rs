use api::api::{ get_run, ApiResponseBuilder};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::get
};
use axum::Extension;
use database::database::select_metrics_from_dimension;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::uuid;

#[sqlx::test(fixtures("../../database/fixtures/metrics.sql"))]
fn it_formats_all_metrics_received_for_a_given_run_into_api_response_struct(
    pool: PgPool,
) -> sqlx::Result<()> {
    let db_connection = pool.acquire().await?;
    let run_id = uuid!("e51076c8-5c47-4a47-a146-04625e77a6ae");

    let project_name = "hubblo/carenage";
    let metrics = select_metrics_from_dimension(db_connection, "run", run_id).await?;

    let formatted_response = ApiResponseBuilder::new(&metrics, project_name).build();

    assert_eq!(formatted_response.processes.len(), 15);
    for process in &formatted_response.processes {
        assert_eq!(process.metrics.len(), 39);
    }
    Ok(())
}

#[sqlx::test(fixtures("../../database/fixtures/metrics.sql"))]
async fn it_returns_a_response_in_json_for_a_given_run_id(db_pool: PgPool) {

    let app = Router::new().route("/runs/:run_id", get(get_run)).layer(Extension(db_pool));

    let run_id = uuid!("e51076c8-5c47-4a47-a146-04625e77a6ae");

    let url = format!("/runs/{run_id}");

    let request = Request::builder().uri(url).body(Body::empty()).unwrap();

    // Need a different client if there is a need to check body data for test assertions.
    let response = app.oneshot(request).await.unwrap(); 

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("../../database/fixtures/metrics.sql"))]
async fn it_returns_an_error_if_database_connection_fails(db_pool: PgPool) {

    // No pool_connection passed to handler. Axum already returns a 500 Internal Servor
    // Error in that case, without a need for explicit error handling. Keeping that test as documentation
    // in case of needing to implement more precise error handling.

    let app = Router::new().route("/runs/:run_id", get(get_run));

    let run_id = uuid!("e51076c8-5c47-4a47-a146-04625e77a6ae");

    let url = format!("/runs/{run_id}");

    let request = Request::builder().uri(url).body(Body::empty()).unwrap();
    let response = app.oneshot(request).await.unwrap(); 

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
