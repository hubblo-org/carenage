use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::with_prefix;
use sqlx::pool::PoolConnection;
use sqlx::types::Uuid;
use sqlx::Postgres;

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
    fn build(&self, component_values: &Value) -> ProcessEmbeddedImpactValues {
        match self {
            ProcessEmbeddedImpacts::Cpu => ProcessEmbeddedImpactValues {
                gwp_average_impact: component_values["gwp_cpu_average_impact"].as_f64().unwrap(),
                gwp_max_impact: component_values["gwp_cpu_max_impact"].as_f64().unwrap(),
                gwp_min_impact: component_values["gwp_cpu_min_impact"].as_f64().unwrap(),
                adp_average_impact: component_values["adp_cpu_average_impact"].as_f64().unwrap(),
                adp_max_impact: component_values["adp_cpu_max_impact"].as_f64().unwrap(),
                adp_min_impact: component_values["adp_cpu_min_impact"].as_f64().unwrap(),
                pe_average_impact: component_values["pe_cpu_average_impact"].as_f64().unwrap(),
                pe_max_impact: component_values["pe_cpu_max_impact"].as_f64().unwrap(),
                pe_min_impact: component_values["pe_cpu_min_impact"].as_f64().unwrap(),
            },
            ProcessEmbeddedImpacts::Ram => ProcessEmbeddedImpactValues {
                gwp_average_impact: component_values["gwp_ram_average_impact"].as_f64().unwrap(),
                gwp_max_impact: component_values["gwp_ram_max_impact"].as_f64().unwrap(),
                gwp_min_impact: component_values["gwp_ram_min_impact"].as_f64().unwrap(),
                adp_average_impact: component_values["adp_ram_average_impact"].as_f64().unwrap(),
                adp_max_impact: component_values["adp_ram_max_impact"].as_f64().unwrap(),
                adp_min_impact: component_values["adp_ram_min_impact"].as_f64().unwrap(),
                pe_average_impact: component_values["pe_ram_average_impact"].as_f64().unwrap(),
                pe_max_impact: component_values["pe_ram_max_impact"].as_f64().unwrap(),
                pe_min_impact: component_values["pe_ram_min_impact"].as_f64().unwrap(),
            },
            ProcessEmbeddedImpacts::Ssd => ProcessEmbeddedImpactValues {
                gwp_average_impact: component_values["gwp_ssd_average_impact"].as_f64().unwrap(),
                gwp_max_impact: component_values["gwp_ssd_max_impact"].as_f64().unwrap(),
                gwp_min_impact: component_values["gwp_ssd_min_impact"].as_f64().unwrap(),
                adp_average_impact: component_values["adp_ssd_average_impact"].as_f64().unwrap(),
                adp_max_impact: component_values["adp_ssd_max_impact"].as_f64().unwrap(),
                adp_min_impact: component_values["adp_ssd_min_impact"].as_f64().unwrap(),
                pe_average_impact: component_values["pe_ssd_average_impact"].as_f64().unwrap(),
                pe_max_impact: component_values["pe_ssd_max_impact"].as_f64().unwrap(),
                pe_min_impact: component_values["pe_ssd_min_impact"].as_f64().unwrap(),
            },
            ProcessEmbeddedImpacts::Hdd => ProcessEmbeddedImpactValues {
                gwp_average_impact: component_values["gwp_hdd_average_impact"].as_f64().unwrap(),
                gwp_max_impact: component_values["gwp_hdd_max_impact"].as_f64().unwrap(),
                gwp_min_impact: component_values["gwp_hdd_min_impact"].as_f64().unwrap(),
                adp_average_impact: component_values["adp_hdd_average_impact"].as_f64().unwrap(),
                adp_max_impact: component_values["adp_hdd_max_impact"].as_f64().unwrap(),
                adp_min_impact: component_values["adp_hdd_min_impact"].as_f64().unwrap(),
                pe_average_impact: component_values["pe_hdd_average_impact"].as_f64().unwrap(),
                pe_max_impact: component_values["pe_hdd_max_impact"].as_f64().unwrap(),
                pe_min_impact: component_values["pe_hdd_min_impact"].as_f64().unwrap(),
            },
        }
    }
}

with_prefix!(prefix_cpu "cpu_");
with_prefix!(prefix_ram "ram_");
with_prefix!(prefix_ssd "ssd_");
with_prefix!(prefix_hdd "hdd_");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(flatten, with = "prefix_cpu")]
    pub process_cpu_embedded_impacts: Option<ProcessEmbeddedImpactValues>,
    #[serde(flatten, with = "prefix_ram")]
    pub process_ram_embedded_impacts: Option<ProcessEmbeddedImpactValues>,
    #[serde(flatten, with = "prefix_ssd")]
    pub process_ssd_embedded_impacts: Option<ProcessEmbeddedImpactValues>,
    #[serde(flatten, with = "prefix_hdd")]
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
    pub fn build(process_data: &Value, boagent_response: &Value) -> Self {
        let pid = process_data.get("pid").expect("PID should be present.");

        let queried_process: Vec<&Value> = boagent_response["raw_data"]["power_data"]["raw_data"]
            .as_array()
            .expect("Data from Scaphandre should be parsable.")
            .last()
            .expect("Last timestamp from Scaphandre should be parsable.")
            .get("consumers")
            .expect("Data on processes should be present from Scaphandre.")
            .as_array()
            .expect("Data on processes should be parsable.")
            .iter()
            .filter(|&process| process["pid"] == *pid)
            .collect();

        let resources = queried_process[0]["resources_usage"]
            .as_object()
            .expect("Data on ressources usage by process should be present.");

        let process_embedded_impacts = process_data
            .get("process_embedded_impacts")
            .expect("Process embedded impacts should be present.");

        let process_ssd_embedded_impacts = process_embedded_impacts
            .get("process_ssd_embedded_impact_values")
            .map(|component_values| ProcessEmbeddedImpacts::Ssd.build(component_values));
        let process_hdd_embedded_impacts = process_embedded_impacts
            .get("process_hdd_embedded_impact_values")
            .map(|component_values| ProcessEmbeddedImpacts::Hdd.build(component_values));

        Metrics {
            process_cpu_embedded_impacts: Some(
                ProcessEmbeddedImpacts::Cpu.build(
                    process_embedded_impacts
                        .get("process_cpu_embedded_impact_values")
                        .expect("CPU embedded impacts for process should be present"),
                ),
            ),
            process_ram_embedded_impacts: Some(
                ProcessEmbeddedImpacts::Ram.build(
                    process_embedded_impacts
                        .get("process_ram_embedded_impact_values")
                        .expect("RAM embedded impacts for process should be present"),
                ),
            ),
            process_ssd_embedded_impacts,
            process_hdd_embedded_impacts,
            cpu_usage_percentage: resources
                .get("cpu_usage")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            memory_usage_bytes: resources
                .get("memory_usage")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            memory_virtual_usage_bytes: resources
                .get("memory_virtual_usage")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            disk_usage_write_bytes: resources
                .get("disk_usage_write")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            disk_usage_read_bytes: resources
                .get("disk_usage_read")
                .unwrap()
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            total_operational_emission_kgc02eq: boagent_response["total_operational_emissions"]
                ["value"]["value"]
                .as_f64()
                .unwrap(),
            total_operational_abiotic_resources_depletion_kgsbeq: boagent_response
                ["total_operational_abiotic_resources_depletion"]["value"]["value"]
                .as_f64()
                .unwrap(),
            total_primary_energy_consumed_mj: boagent_response
                ["total_operational_primary_energy_consumed"]["value"]["value"]
                .as_f64()
                .unwrap(),
            embedded_emissions_kgc02eq: boagent_response["embedded_emissions"]["value"]
                .as_f64()
                .unwrap(),
            embedded_abiotic_resources_depletion_kgsbeq: boagent_response
                ["embedded_abiotic_resources_depletion"]["value"]
                .as_f64()
                .unwrap(),
            embedded_primary_energy_mj: boagent_response["embedded_primary_energy"]["value"]
                .as_f64()
                .unwrap(),
            average_power_measured_w: boagent_response["average_power_measured"]["value"]
                .as_f64()
                .unwrap(),
        }
    }
    pub async fn insert(
        &self,
        event_id: Uuid,
        db_connection: PoolConnection<Postgres>,
    ) -> Result<(), sqlx::Error> {
        let mut connection = db_connection.detach();

        let metrics_value = serde_json::to_value(self).expect("Metrics should be deserializable.");
        let iterable_metrics = metrics_value
            .as_object()
            .expect("Metrics should be parsable.");

        let metric_fields: Vec<String> = iterable_metrics
            .iter()
            .map(|metric| metric.0.clone())
            .collect();

        let metric_values: Vec<f64> = iterable_metrics
            .iter()
            .map(|metric| metric.1.as_f64().unwrap())
            .collect();

        let query = "INSERT INTO METRICS (event_id, metric, value) VALUES ($1, UNNEST($2::VARCHAR(255)[]), UNNEST($3::NUMERIC[]))";

        sqlx::query(query)
            .bind(event_id)
            .bind(metric_fields)
            .bind(metric_values)
            .execute(&mut connection)
            .await?;

        println!("Inserted metrics.");
        Ok(())
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
