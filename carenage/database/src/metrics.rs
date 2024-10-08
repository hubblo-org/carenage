use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::database::Process;

pub trait Metric {
    fn build(&self, process_embedded_impacts: &Value) -> ProcessEmbeddedImpactValues;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessEmbeddedImpacts {
    Cpu,
    Ram,
    Ssd,
    Hdd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEmbeddedImpactValues {
    pub gwp_average_impact: f64,
    pub gwp_max_impact: f64,
    pub gwp_min_impact: f64,
    pub adp_average_impact: f64,
    pub adp_max_impact: f64,
    pub adp_min_impact: f64,
    pub pe_average_impact: f64,
    pub pe_max_impact: f64,
    pub pe_min_impact: f64,
}

impl Metric for ProcessEmbeddedImpacts {
    fn build(&self, process_embedded_impacts: &Value) -> ProcessEmbeddedImpactValues {
        let components_values = &process_embedded_impacts["process_embedded_impacts"];
        match self {
            ProcessEmbeddedImpacts::Cpu => {
                let cpu_values = &components_values["process_cpu_embedded_impact_values"];
                ProcessEmbeddedImpactValues {
                    gwp_average_impact: cpu_values["gwp_cpu_average_impact"].as_f64().unwrap(),
                    gwp_max_impact: cpu_values["gwp_cpu_max_impact"].as_f64().unwrap(),
                    gwp_min_impact: cpu_values["gwp_cpu_min_impact"].as_f64().unwrap(),
                    adp_average_impact: cpu_values["adp_cpu_average_impact"].as_f64().unwrap(),
                    adp_max_impact: cpu_values["adp_cpu_max_impact"].as_f64().unwrap(),
                    adp_min_impact: cpu_values["adp_cpu_min_impact"].as_f64().unwrap(),
                    pe_average_impact: cpu_values["pe_cpu_average_impact"].as_f64().unwrap(),
                    pe_max_impact: cpu_values["pe_cpu_max_impact"].as_f64().unwrap(),
                    pe_min_impact: cpu_values["pe_cpu_min_impact"].as_f64().unwrap(),
                }
            }
            ProcessEmbeddedImpacts::Ram => {
                let ram_values = &components_values["process_ram_embedded_impact_values"];
                ProcessEmbeddedImpactValues {
                    gwp_average_impact: ram_values["gwp_ram_average_impact"].as_f64().unwrap(),
                    gwp_max_impact: ram_values["gwp_ram_max_impact"].as_f64().unwrap(),
                    gwp_min_impact: ram_values["gwp_ram_min_impact"].as_f64().unwrap(),
                    adp_average_impact: ram_values["adp_ram_average_impact"].as_f64().unwrap(),
                    adp_max_impact: ram_values["adp_ram_max_impact"].as_f64().unwrap(),
                    adp_min_impact: ram_values["adp_ram_min_impact"].as_f64().unwrap(),
                    pe_average_impact: ram_values["pe_ram_average_impact"].as_f64().unwrap(),
                    pe_max_impact: ram_values["pe_ram_max_impact"].as_f64().unwrap(),
                    pe_min_impact: ram_values["pe_ram_min_impact"].as_f64().unwrap(),
                }
            }
            ProcessEmbeddedImpacts::Ssd => {
                let ssd_values = &components_values["process_ssd_embedded_impact_values"];
                ProcessEmbeddedImpactValues {
                    gwp_average_impact: ssd_values["gwp_ssd_average_impact"].as_f64().unwrap(),
                    gwp_max_impact: ssd_values["gwp_ssd_max_impact"].as_f64().unwrap(),
                    gwp_min_impact: ssd_values["gwp_ssd_min_impact"].as_f64().unwrap(),
                    adp_average_impact: ssd_values["adp_ssd_average_impact"].as_f64().unwrap(),
                    adp_max_impact: ssd_values["adp_ssd_max_impact"].as_f64().unwrap(),
                    adp_min_impact: ssd_values["adp_ssd_min_impact"].as_f64().unwrap(),
                    pe_average_impact: ssd_values["pe_ssd_average_impact"].as_f64().unwrap(),
                    pe_max_impact: ssd_values["pe_ssd_max_impact"].as_f64().unwrap(),
                    pe_min_impact: ssd_values["pe_ssd_min_impact"].as_f64().unwrap(),
                }
            }
            ProcessEmbeddedImpacts::Hdd => {
                let hdd_values = &components_values["process_hdd_embedded_impact_values"];

                ProcessEmbeddedImpactValues {
                    gwp_average_impact: hdd_values["gwp_hdd_average_impact"].as_f64().unwrap(),
                    gwp_max_impact: hdd_values["gwp_hdd_max_impact"].as_f64().unwrap(),
                    gwp_min_impact: hdd_values["gwp_hdd_min_impact"].as_f64().unwrap(),
                    adp_average_impact: hdd_values["adp_hdd_average_impact"].as_f64().unwrap(),
                    adp_max_impact: hdd_values["adp_hdd_max_impact"].as_f64().unwrap(),
                    adp_min_impact: hdd_values["adp_hdd_min_impact"].as_f64().unwrap(),
                    pe_average_impact: hdd_values["pe_hdd_average_impact"].as_f64().unwrap(),
                    pe_max_impact: hdd_values["pe_hdd_max_impact"].as_f64().unwrap(),
                    pe_min_impact: hdd_values["pe_hdd_min_impact"].as_f64().unwrap(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub process_cpu_embedded_impacts: ProcessEmbeddedImpactValues,
    pub process_ram_embedded_impacts: ProcessEmbeddedImpactValues,
    pub process_ssd_embedded_impacts: Option<ProcessEmbeddedImpactValues>,
    pub process_hdd_embedded_impacts: Option<ProcessEmbeddedImpactValues>,
    pub cpu_usage_percentage: f64,
    pub memory_usage_bytes: u64,
    pub memory_virtual_usage_bytes: u64,
    pub disk_usage_write_bytes: u64,
    pub disk_usage_read_bytes: u64,
    pub total_operational_emission_kgc02eq: f64,
    pub total_operational_abiotic_resources_depletion_kgsbeq: f64,
    pub total_primary_energy_consumed_mj: f64,
    pub average_power_measured_w: f64,
    pub embedded_emissions_kgc02eq: f64,
    pub embedded_abiotic_resources_depletion_kgsbeq: f64,
    pub embedded_primary_energy_mj: f64,
}

impl Metrics {
    pub fn build(
        process_embedded_impacts: Value,
        boagent_response: Value,
    ) -> Result<Metrics, Box<dyn std::error::Error>> {
        let pid = process_embedded_impacts
            .get("pid")
            .expect("PID should be present.");

        let last_timestamp = boagent_response["raw_data"]["power_data"]["raw_data"]
            .as_array()
            .expect("Data from Scaphandre should be parsable.")
            .last()
            .expect("Last timestamp from Scaphandre should be parsable.");

        let processes = last_timestamp["consumers"]
            .as_array()
            .expect("Processes should be parsable from Scaphandre")
            .iter();
        let queried_process: Process = processes.filter(|process| process["pid"] == *pid).collect();
        // let process = serde_json::from_slice(queried_process);

        Ok(Metrics {
            process_cpu_embedded_impacts: ProcessEmbeddedImpacts::Cpu
                .build(&process_embedded_impacts),
            process_ram_embedded_impacts: ProcessEmbeddedImpacts::Ram
                .build(&process_embedded_impacts),
            process_ssd_embedded_impacts: Some(
                ProcessEmbeddedImpacts::Ssd.build(&process_embedded_impacts),
            ),
            process_hdd_embedded_impacts: Some(
                ProcessEmbeddedImpacts::Hdd.build(&process_embedded_impacts),
            ),
            cpu_usage_percentage: queried_process["cpu_usage"].as_f64().unwrap(),
            memory_usage_bytes: queried_process["memory_usage"].as_u64().unwrap(),
            memory_virtual_usage_bytes: queried_process["memory_virtual_usage_unit"]
                .as_u64()
                .unwrap(),
            disk_usage_write_bytes: queried_process["disk_usage_write"].as_u64().unwrap(),
            disk_usage_read_bytes: queried_process["disk_usage_read"].as_u64().unwrap(),
            total_operational_emission_kgc02eq: boagent_response["total_operational_emissions"]
                .as_f64()
                .unwrap(),
            total_operational_abiotic_resources_depletion_kgsbeq: boagent_response
                ["total_operational_abiotic_resources_depltion"]
                .as_f64()
                .unwrap(),
            total_primary_energy_consumed_mj: boagent_response
                ["total_operational_primary_energy_consumed"]
                .as_f64()
                .unwrap(),
            embedded_emissions_kgc02eq: boagent_response["embedded_emissions"].as_f64().unwrap(),
            embedded_abiotic_resources_depletion_kgsbeq: boagent_response
                ["embedded_abiotic_resources_depletion"]
                .as_f64()
                .unwrap(),
            embedded_primary_energy_mj: boagent_response["embedded_primary_energy"]
                .as_f64()
                .unwrap(),
            average_power_measured_w: boagent_response["average_power_measured"].as_f64().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_builds_serialized_process_embedded_impacts_from_a_json_value() {
        let data = json!({
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
                },
                "process_hdd_embedded_impact_values": {
                    "gwp_hdd_average_impact": 0.0000021533829645868956,
                    "adp_hdd_average_impact": 7.321502079595447e-11,
                    "pe_hdd_average_impact": 0.00002584059557504275,
                    "gwp_hdd_max_impact": 0.0003843788591787609,
                    "adp_hdd_max_impact": 1.3068881212077872e-8,
                    "pe_hdd_max_impact": 0.004612546310145131,
                    "gwp_hdd_min_impact": 0,
                    "adp_hdd_min_impact": 0,
                    "pe_hdd_min_impact": 0
                }
            }
        });
        let process_cpu_embedded_impacts = ProcessEmbeddedImpacts::Cpu.build(&data);
        let process_ram_embedded_impacts = ProcessEmbeddedImpacts::Ram.build(&data);
        let process_ssd_embedded_impacts = ProcessEmbeddedImpacts::Ssd.build(&data);
        let process_hdd_embedded_impacts = ProcessEmbeddedImpacts::Hdd.build(&data);

        assert_eq!(
            process_cpu_embedded_impacts.gwp_average_impact,
            0.38336191697478994
        );
        assert_eq!(
            process_ram_embedded_impacts.gwp_average_impact,
            6.628147200042126
        );
        assert_eq!(
            process_ssd_embedded_impacts.gwp_average_impact,
            0.0000021533829645868956
        );
        assert_eq!(
            process_hdd_embedded_impacts.gwp_average_impact,
            0.0000021533829645868956
        );
    }
}
