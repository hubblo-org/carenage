use database::timestamp::UnixFlag;
use database::{
    deserialize_boagent_json, format_hardware_data, get_db_connection_pool, insert_device_metadata,
    insert_dimension_table_metadata, query_boagent, timestamp::Timestamp,
};
use serde_json::json;
use std::env;
use std::process;

pub struct DaemonArgs {
    pub time_step: u64,
    pub start_timestamp: Timestamp,
    pub unix_flag: UnixFlag,
    pub init_flag: bool,
}

impl DaemonArgs {
    pub fn parse_args() -> Result<DaemonArgs, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        let time_step: u64 = args[1]
            .parse()
            .expect("time_step variable should be parsable.");
        let start_time_str = args[2].to_string();
        let is_unix_set: bool = args[3]
            .parse()
            .expect("is_unix_set variable should be parsable.");
        let init_flag: bool = args[4]
            .parse()
            .expect("is_init_set variable should be parsable.");
        let unix_flag = UnixFlag::from_bool(is_unix_set);
        let start_timestamp = Timestamp::parse_str(start_time_str, unix_flag);

        Ok(DaemonArgs {
            time_step,
            start_timestamp,
            unix_flag,
            init_flag,
        })
    }
}
