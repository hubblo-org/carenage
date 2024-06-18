CREATE TABLE projects (
  project_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE repositories (
  repository_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE workflows (
  workflow_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE pipelines (
  pipeline_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE runs (
  run_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE jobs (
  job_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE tasks (
  task_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE containers (
  container_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE processes (
  process_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  container_id uuid REFERENCES containers(container_id),
  exe VARCHAR(255),
  cmdline TEXT,
  state VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP
);

CREATE TABLE devices (
  device_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  lifetime INTEGER,
  location CHARACTER(3)
);

CREATE TABLE components (
  component_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  machine_id uuid REFERENCES devices(device_id),
  name VARCHAR(255),
  model VARCHAR(255),
  manufacturer VARCHAR(255)
);

CREATE TABLE component_characteristic (
  component_id uuid REFERENCES components(component_id),
  name VARCHAR(255),
  value VARCHAR(255)
);

CREATE TABLE events (
  timestamp TIMESTAMP,
  process_id uuid REFERENCES processes(process_id),
  task_id uuid REFERENCES tasks(task_id),
  job_id uuid  REFERENCES jobs(job_id),
  run_id uuid  REFERENCES runs(run_id),
  pipeline_id uuid  REFERENCES pipelines(pipeline_id),
  workflow_id uuid  REFERENCES workflows(workflow_id),
  repository_id uuid  REFERENCES repositories(repository_id),
  project_id uuid  REFERENCES projects(project_id),
  machine_id uuid  REFERENCES devices(device_id),
  type VARCHAR(255),
  user_label TEXT,
  CPU_time_percent FLOAT,
  RAM_virt_bytes INTEGER,
  RAM_phy_bytes INTEGER,
  DISK_write_bytes INTEGER,
  PE_emb_min_process FLOAT,
  PE_use_value_process FLOAT,
  PE_use_max_process FLOAT,
  PE_use_min_process FLOAT,
  GWP_emb_value_component FLOAT,
  GWP_emb_max_component FLOAT,
  GWP_emb_min_component FLOAT,
  GWP_use_value_component FLOAT,
  GWP_use_max_component FLOAT,
  GWP_use_min_component FLOAT,
  ADP_emb_value_component FLOAT,
  ADP_emb_max_component FLOAT,
  ADP_emb_min_component FLOAT,
  ADP_use_value_component FLOAT,
  ADP_use_max_component FLOAT,
  ADP_use_min_component FLOAT,
  PE_emb_value_component FLOAT,
  PE_emb_max_component FLOAT,
  PE_emb_min_component FLOAT,
  PE_use_value_component FLOAT,
  PE_use_max_component FLOAT,
  PE_use_min_component FLOAT,
  CONSTRAINT primary_keys PRIMARY KEY (task_id, job_id, run_id, pipeline_id, workflow_id, repository_id, project_id, machine_id)
);
