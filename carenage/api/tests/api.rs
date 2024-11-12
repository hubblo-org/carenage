use api::api::{ get_run, ApiResponseBuilder};
use axum::{
    body::Body,
    http::{Request, StatusCode}, response::IntoResponse,
    Router,
    routing::get
};
use axum::Extension;
use database::database::select_metrics_from_dimension;
use sqlx::PgPool;
use tower::{ Service, ServiceExt};
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

    let project_name = "carenage/webapp".to_owned();

    let app = Router::new().route("/runs/:run_id", get(get_run)).layer(Extension(db_pool)).layer(Extension(project_name));

    let run_id = uuid!("e51076c8-5c47-4a47-a146-04625e77a6ae");

    let url = format!("/runs/{run_id}");

    let response = app.oneshot(Request::builder().uri(url).body(Body::empty()).unwrap()).await.unwrap(); 


    println!("{:?}", response);
}
