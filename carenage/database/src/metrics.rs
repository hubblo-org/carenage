use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessEmbeddedImpacts {
    pub process_cpu_embedded_impact_values: ProcessCpuEmbeddedImpacts,
    pub process_ram_embedded_impact_values: ProcessRamEmbeddedImpacts,
    pub process_ssd_embedded_impact_values: Option<ProcessSsdEmbeddedImpacts>,
    pub process_hdd_embedded_impact_values: Option<ProcessHddEmbeddedImpacts>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "process_cpu_embedded_impact_values")]
pub struct ProcessCpuEmbeddedImpacts {
    pub gwp_cpu_average_impact: f64,
    pub gwp_cpu_max_impact: f64,
    pub gwp_cpu_min_impact: f64,
    pub adp_cpu_average_impact: f64,
    pub adp_cpu_max_impact: f64,
    pub adp_cpu_min_impact: f64,
    pub pe_cpu_average_impact: f64,
    pub pe_cpu_max_impact: f64,
    pub pe_cpu_min_impact: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "process_ram_embedded_impact_values")]
pub struct ProcessRamEmbeddedImpacts {
    pub gwp_ram_average_impact: f64,
    pub gwp_ram_max_impact: f64,
    pub gwp_ram_min_impact: f64,
    pub adp_ram_average_impact: f64,
    pub adp_ram_max_impact: f64,
    pub adp_ram_min_impact: f64,
    pub pe_ram_average_impact: f64,
    pub pe_ram_max_impact: f64,
    pub pe_ram_min_impact: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "process_ssd_embedded_impact_values")]
pub struct ProcessSsdEmbeddedImpacts {
    pub gwp_ssd_average_impact: f64,
    pub gwp_ssd_max_impact: f64,
    pub gwp_ssd_min_impact: f64,
    pub adp_ssd_average_impact: f64,
    pub adp_ssd_max_impact: f64,
    pub adp_ssd_min_impact: f64,
    pub pe_ssd_average_impact: f64,
    pub pe_ssd_max_impact: f64,
    pub pe_ssd_min_impact: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "process_hdd_embedded_impact_values")]
pub struct ProcessHddEmbeddedImpacts {
    pub gwp_hdd_average_impact: f64,
    pub gwp_hdd_max_impact: f64,
    pub gwp_hdd_min_impact: f64,
    pub adp_hdd_average_impact: f64,
    pub adp_hdd_max_impact: f64,
    pub adp_hdd_min_impact: f64,
    pub pe_hdd_average_impact: f64,
    pub pe_hdd_max_impact: f64,
    pub pe_hdd_min_impact: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub process_embedded_impacts: ProcessEmbeddedImpacts,
}
