use database::metrics::ProcessEmbeddedImpactValues;
use database::{database::Ids, metrics::Metrics};
use log::{debug, info};
use sqlx::postgres::PgRow;
use sqlx::Row;

pub struct ProcessRecord {
    pub pid: i32,
    pub exe: String,
    pub cmdline: String,
    pub metrics: Metrics,
}
pub struct ApiResponse {
    pub project_name: String,
    pub ids: Ids,
    pub processes: Vec<ProcessRecord>,
}

pub struct ApiResponseBuilder(ApiResponse);

pub struct ProcessRecordBuilder(ProcessRecord);

impl ProcessRecordBuilder {
    pub fn new(metric_row: &PgRow) -> Self {
        let process_cpu_embedded_impacts = ProcessEmbeddedImpactValues {
            adp_max_impact: metric_row.get("cpu_adp_max_impact"),
            adp_min_impact: metric_row.get("cpu_adp_min_impact"),
            adp_average_impact: metric_row.get("cpu_adp_average_impact"),
            gwp_max_impact: metric_row.get("cpu_gwp_max_impact"),
            gwp_min_impact: metric_row.get("cpu_gwp_min_impact"),
            gwp_average_impact: metric_row.get("cpu_gwp_average_impact"),
            pe_max_impact: metric_row.get("cpu_pe_max_impact"),
            pe_min_impact: metric_row.get("cpu_pe_min_impact"),
            pe_average_impact: metric_row.get("cpu_pe_average_impact"),
        };
        let process_ram_embedded_impacts = ProcessEmbeddedImpactValues {
            adp_max_impact: metric_row.get("ram_adp_max_impact"),
            adp_min_impact: metric_row.get("ram_adp_min_impact"),
            adp_average_impact: metric_row.get("ram_adp_average_impact"),
            gwp_max_impact: metric_row.get("ram_gwp_max_impact"),
            gwp_min_impact: metric_row.get("ram_gwp_min_impact"),
            gwp_average_impact: metric_row.get("ram_gwp_average_impact"),
            pe_max_impact: metric_row.get("ram_pe_max_impact"),
            pe_min_impact: metric_row.get("ram_pe_min_impact"),
            pe_average_impact: metric_row.get("ram_pe_average_impact"),
        };

        let process_ssd_embedded_impacts = match metric_row.try_get("ssd_adp_max_impact") {
            Ok(value) => Some(ProcessEmbeddedImpactValues {
                adp_max_impact: value,
                adp_min_impact: metric_row.get("ssd_adp_min_impact"),
                adp_average_impact: metric_row.get("ssd_adp_average_impact"),
                gwp_max_impact: metric_row.get("ssd_gwp_max_impact"),
                gwp_min_impact: metric_row.get("ssd_gwp_min_impact"),
                gwp_average_impact: metric_row.get("ssd_gwp_average_impact"),
                pe_max_impact: metric_row.get("ssd_pe_max_impact"),
                pe_min_impact: metric_row.get("ssd_pe_min_impact"),
                pe_average_impact: metric_row.get("ssd_pe_average_impact"),
            }),
            Err(error) => {
                info!("No SSD information for this process!");
                debug!("{}", error);
                None
            }
        };

        let process_hdd_embedded_impacts = match metric_row.try_get("hdd_adp_max_impact") {
            Ok(value) => Some(ProcessEmbeddedImpactValues {
                adp_max_impact: value,
                adp_min_impact: metric_row.get("hdd_adp_min_impact"),
                adp_average_impact: metric_row.get("hdd_adp_average_impact"),
                gwp_max_impact: metric_row.get("hdd_gwp_max_impact"),
                gwp_min_impact: metric_row.get("hdd_gwp_min_impact"),
                gwp_average_impact: metric_row.get("hdd_gwp_average_impact"),
                pe_max_impact: metric_row.get("hdd_pe_max_impact"),
                pe_min_impact: metric_row.get("hdd_pe_min_impact"),
                pe_average_impact: metric_row.get("hdd_pe_average_impact"),
            }),
            Err(error) => {
                info!("No HDD information for this process!");
                debug!("{}", error);
                None
            }
        };

        let process_metrics = Metrics {
            average_power_measured_w: metric_row.get("average_power_measured_w"),
            cpu_usage_percentage: metric_row.get("cpu_usage_percentage"),
            disk_usage_read_bytes: metric_row.get("disk_usage_read_bytes"),
            disk_usage_write_bytes: metric_row.get("disk_usage_write_bytes"),
            embedded_emissions_kgc02eq: metric_row.get("embedded_emissions_kgc02eq"),
            embedded_abiotic_resources_depletion_kgsbeq: metric_row
                .get("embedded_abiotic_resources_depletion_kgsbeq"),
            embedded_primary_energy_mj: metric_row.get("embedded_primary_energy_mj"),
            memory_usage_bytes: metric_row.get("memory_usage_bytes"),
            memory_virtual_usage_bytes: metric_row.get("memory_virtual_usage_bytes"),
            process_cpu_embedded_impacts: Some(process_cpu_embedded_impacts),
            process_ram_embedded_impacts: Some(process_ram_embedded_impacts),
            process_ssd_embedded_impacts,
            process_hdd_embedded_impacts,
            total_operational_abiotic_resources_depletion_kgsbeq: metric_row
                .get("total_operational_abiotic_resources_depletion_kgsbeq"),
            total_operational_emission_kgc02eq: metric_row
                .get("total_operational_emission_kgc02eq"),
            total_primary_energy_consumed_mj: metric_row.get("total_primary_energy_consumed_mj"),
        };

        ProcessRecordBuilder(ProcessRecord {
            pid: metric_row.get("pid"),
            exe: metric_row.get("exe"),
            cmdline: metric_row.get("cmdline"),
            metrics: process_metrics,
        })
    }
    pub fn build(self) -> ProcessRecord {
        self.0
    }
}

impl ApiResponseBuilder {
    pub fn new(metrics_rows: &[PgRow]) -> Self {
        let processes_records: Vec<ProcessRecord> = metrics_rows
            .iter()
            .map(|metric_row| ProcessRecordBuilder::new(metric_row).build())
            .collect();
        ApiResponseBuilder(ApiResponse {
            processes: processes_records,
            project_name: todo!(),
            ids: todo!(),
        })
    }
    pub fn build(self) -> ApiResponse {
        self.0
    }
}
