CREATE TABLE projects (
  project_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE repositories (
  repository_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE workflows (
  workflow_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE pipelines (
  pipeline_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE runs (
  run_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE jobs (
  job_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE tasks (
  task_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE containers (
  container_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE processes (
  process_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  container_id uuid REFERENCES containers(container_id),
  exe VARCHAR(255),
  cmdline TEXT,
  state VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE devices (
  device_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  lifetime INTEGER,
  location CHARACTER(3)
);

CREATE TABLE components (
  component_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  device_id uuid REFERENCES devices(device_id),
  name VARCHAR(255),
  model VARCHAR(255),
  manufacturer VARCHAR(255)
);

CREATE TABLE component_characteristic (
  component_id uuid REFERENCES components(component_id),
  name VARCHAR(255),
  value VARCHAR(255)
);

CREATE TYPE event_type AS ENUM ('regular', 'custom', 'start', 'stop');

CREATE TABLE events (
  timestamp TIMESTAMPTZ,
  process_id uuid REFERENCES processes(process_id),
  task_id uuid REFERENCES tasks(task_id),
  job_id uuid REFERENCES jobs(job_id),
  run_id uuid REFERENCES runs(run_id),
  pipeline_id uuid REFERENCES pipelines(pipeline_id),
  workflow_id uuid REFERENCES workflows(workflow_id),
  repository_id uuid REFERENCES repositories(repository_id),
  project_id uuid REFERENCES projects(project_id),
  device_id uuid REFERENCES devices(device_id),
  event_type event_type,
  user_label TEXT,
  CPU_time_percent FLOAT,
  RAM_virt_bytes INTEGER,
  RAM_phy_bytes INTEGER,
  DISK_write_bytes INTEGER,
  GWP_emb_value_process FLOAT,
  GWP_emb_max_process FLOAT,
  GWP_emb_min_process FLOAT,
  GWP_use_value_process FLOAT,
  GWP_use_max_process FLOAT,
  GWP_use_min_process FLOAT,
  ADP_emb_value_process FLOAT,
  ADP_emb_max_process FLOAT,
  ADP_emb_min_process FLOAT,
  ADP_use_value_process FLOAT,
  ADP_use_max_process FLOAT,
  ADP_use_min_process FLOAT,
  PE_emb_value_process FLOAT,
  PE_emb_max_process FLOAT,
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
  GWP_total_operational_emissions FLOAT,
  GWP_total_embedded_emissions FLOAT,
  GWP_total FLOAT,
  ADP_total_operational_emissions FLOAT,
  ADP_total_embedded_emissions FLOAT,
  ADP_total FLOAT,
  PE_total_operational_emissions FLOAT,
  PE_total_embedded_emissions FLOAT,
  PE_total FLOAT,
  total_time INTEGER,
  avg_power FLOAT,
  electric_carbon_intensity FLOAT,
  CONSTRAINT primary_keys PRIMARY KEY (task_id, job_id, run_id, pipeline_id, workflow_id, repository_id, project_id, device_id)
);
