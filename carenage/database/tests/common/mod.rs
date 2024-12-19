use serde_json::json;

pub fn process_data() -> serde_json::Value {
    let process_data = json!({
        "pid": 6042,
        "process_embedded_impacts": {
            "process_cpu_embedded_impact_values": {
                "gwp_cpu_average_impact": 0.38336191697478994,
                "adp_cpu_average_impact": 0.00039112499579352936,
                "pe_cpu_average_impact": 5.750428754621844,
                "gwp_cpu_max_impact": 0.9255784800000001,
                "adp_cpu_max_impact": 0.00094432144422,
                "pe_cpu_max_impact": 13.883677200000001,
                "gwp_cpu_min_impact": 0.0049683272,
                "adp_cpu_min_impact": 0.0000050689358258,
                "pe_cpu_min_impact": 0.074524908
            },
            "process_ram_embedded_impact_values": {
                "gwp_ram_average_impact": 6.628147200042126,
                "adp_ram_average_impact": 0.0003516976065328474,
                "pe_ram_average_impact": 81.16098612296481,
                "gwp_ram_max_impact": 13.492131233215332,
                "adp_ram_max_impact": 0.0007159090042114257,
                "pe_ram_max_impact": 165.20977020263672,
                "gwp_ram_min_impact": 0,
                "adp_ram_min_impact": 0,
                "pe_ram_min_impact": 0
            },
            "process_ssd_embedded_impact_values": {
                "gwp_ssd_average_impact": 0.0000021533829645868956,
                "adp_ssd_average_impact": 7.321502079595447e-11,
                "pe_ssd_average_impact": 0.00002584059557504275,
                "gwp_ssd_max_impact": 0.0003843788591787609,
                "adp_ssd_max_impact": 1.3068881212077872e-8,
                "pe_ssd_max_impact": 0.004612546310145131,
                "gwp_ssd_min_impact": 0,
                "adp_ssd_min_impact": 0,
                "pe_ssd_min_impact": 0
            }
        }
    });
    process_data
}
