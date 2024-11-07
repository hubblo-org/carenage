use database::database::select_metrics_from_dimension;
use api::api::ApiResponseBuilder;
use sqlx::PgPool;
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
    for process in formatted_response.processes {
        assert_eq!(process.metrics.len(), 39);
    }
    Ok(())
}
