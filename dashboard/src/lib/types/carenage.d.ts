export declare type ProcessInfo = {
  process_pid: number;
  process_exe: string;
  process_cmdline: string;
};

export declare type MetricValues = (string | number)[][];

export declare type Metric = {
  metric_name: string;
  metric_values: MetricValues;
};

export declare type Process = {
  process: ProcessInfo;
  metrics: Metric[];
};

export declare type CiRunMetadata = {
  run_id: string;
  run_repo_id: number;
  run_repo_url: string;
  started_at: string;
  finished_at: string;
  duration: number;
};

export declare type CiRun = CiRunMetadata & {
  pipeline_id: string;
  pipeline_repo_id: number;
  pipeline_repo_url: string;
  project_name: string;
  project_repo_url: string;
  job_name: string;
  job_status: string;
  processes: Process[];
};

export declare type CiPipelineMetadata = {
  pipeline_id: string;
  pipeline_repo_id: number;
  pipeline_repo_url: string;
  project_name: string;
  project_repo_url: string;
  started_at: string;
  finished_at: string;
  duration: number;
  runs: CiRunMetadata[];
};

export declare type CiPipeline = CiPipelineMetadata & {
  runs: CiRun[];
};

export declare type ProjectMetadata = {
  project_id: string;
  project_name: string;
  project_url: string;
  project_repo_id: number;
  created_at: string;
  pipelines: CiPipelineMetadata[];
};
declare module "carenage";
