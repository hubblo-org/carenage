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

export declare type CiRun = {
  pipeline_id: number;
  run_id: number;
  project_name: string;
  started_at: string;
  job_name: string;
  job_status: string;
  job_duration: number;
  processes: Process[];
};

export declare type CiPipeline = {
  pipeline_id: string;
  pipeline_repo_id: number;
  pipeline_repo_url: string;
  started_at: string;
  finished_at: string;
  duration: number;
  runs: CiRun[];
};

export declare type ProjectRecord = {
  project_id: string;
  project_name: string;
  project_url: string;
  project_repo_id: number;
  created_at: string;
  pipelines: CiPipeline[];
};
declare module "carenage";
