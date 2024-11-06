use database::database::Record;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessInfo {
    pub process_pid: i32,
    pub process_exe: String,
    pub process_cmdline: String,
}

#[derive(Debug)]
pub struct ProcessMetrics {
    pub metric_name: String,
    pub metric_values: Vec<f64>,
}

#[derive(Debug)]
pub struct ProcessRecord {
    pub process: ProcessInfo,
    pub metrics: Vec<ProcessMetrics>,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub project_name: String,
    pub processes: Vec<ProcessRecord>,
}

pub struct ApiResponseBuilder(ApiResponse);

impl ApiResponseBuilder {
    pub fn new(records: &[Record], project_name: String) -> Self {
        let mut metrics_names: Vec<String> = records
            .iter()
            .map(|record| record.metric.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        let mut processes_infos: Vec<ProcessInfo> = records
            .iter()
            .map(|record| ProcessInfo {
                process_pid: record.pid,
                process_exe: record.exe.clone(),
                process_cmdline: record.cmdline.clone(),
            })
            .collect::<HashSet<ProcessInfo>>()
            .into_iter()
            .collect();

        processes_infos.sort();
        metrics_names.sort();

        let processes: Vec<ProcessRecord> = processes_infos
            .into_iter()
            .map(|process| {
                let process_metrics: Vec<ProcessMetrics> = metrics_names
                    .clone()
                    .into_iter()
                    .map(|metric_name| {
                        let metric_values = records
                            .iter()
                            .filter(|record| {
                                record.pid == process.process_pid && record.metric == metric_name
                            })
                            .map(|record| record.value)
                            .collect::<Vec<f64>>();

                        ProcessMetrics {
                            metric_name,
                            metric_values,
                        }
                    })
                    .collect();
                ProcessRecord {
                    process,
                    metrics: process_metrics,
                }
            })
            .collect();
        ApiResponseBuilder(ApiResponse {
            project_name,
            processes,
        })
    }

    pub fn build(self) -> ApiResponse {
        self.0
    }
}
