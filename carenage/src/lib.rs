use reqwest::Result;
use reqwest::blocking::Client;

fn query_boagent(start_time: Option<u64>, end_time: Option<u64>, location: String, lifetime: u8) -> Result<u16> {
    let client = Client::new();
    let base_url = format!("http://127.0.0.1:8000/query");    
    let response = client.get(base_url)
                    .query(&[("start_time", start_time.expect("fail").to_string())])   
                    .query(&[("end_time", end_time.expect("fail").to_string())])
                    .query(&[("verbose", "true")])
                    .query(&[("location", location)])
                    .query(&[("measure_power", "true")])
                    .query(&[("lifetime", lifetime)])
                    .query(&[("fetch_hardware", "true")])
                    .send()?;
        
        

    match response.status().as_u16() {
        200 => {
            println!("OK");
            Ok(200)
        }
        _ => {
            println!("Error");
            Ok(response.status().as_u16())
        }
    }
}



#[cfg(test)]
mod tests {
    use mockito::{Matcher, Server};
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn it_queries_boagent_with_success_with_needed_query_paramaters() {

        let now_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let now_timestamp_minus_one_minute = now_timestamp - 60; 
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 8000,
            ..Default::default()
        };

        let mut boagent_server = Server::new_with_opts(opts);

        let mock = boagent_server.mock("GET", "/query")
            .match_query(Matcher::AllOf(vec![
                Matcher::Regex(format!("start_time={:?}", now_timestamp_minus_one_minute).into()),
                Matcher::Regex(format!("end_time={:?}", now_timestamp).into()),
                Matcher::Regex("verbose=true".into()),
                Matcher::Regex("location=FRA".into()),
                Matcher::Regex("measure_power=true".into()),
                Matcher::Regex("lifetime=5".into()),
                Matcher::Regex("fetch_hardware=true".into())
            ]))
            .with_status(200)
            .create();

        let response = query_boagent(Some(now_timestamp_minus_one_minute), Some(now_timestamp), "FRA".to_string(), 5).unwrap(); 
        mock.assert();
        assert_eq!(response, 200);
    }

}
