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
  job_name: string;
  job_status: string;
  job_duration: number;
  processes: Process[];
};

declare module "carenage";
