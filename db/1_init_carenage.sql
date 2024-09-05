CREATE TABLE projects (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) UNIQUE,
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE repositories (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE workflows (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE pipelines (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE runs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE jobs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE tasks (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE containers (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE processes (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  container_id uuid REFERENCES containers(id),
  exe VARCHAR(255),
  cmdline TEXT,
  state VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE devices (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  lifetime INTEGER,
  location CHARACTER(3)
);

CREATE TABLE components (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  device_id uuid REFERENCES devices(id),
  name VARCHAR(255),
  model VARCHAR(255),
  manufacturer VARCHAR(255)
);

CREATE TABLE component_characteristic (
  component_id uuid REFERENCES components(id),
  name VARCHAR(255),
  value VARCHAR(255)
);

CREATE TYPE event_type AS ENUM ('regular', 'custom', 'start', 'stop');

CREATE TABLE events (
  timestamp TIMESTAMPTZ,
  process_id uuid REFERENCES processes(id),
  task_id uuid REFERENCES tasks(id),
  job_id uuid REFERENCES jobs(id),
  run_id uuid REFERENCES runs(id),
  pipeline_id uuid REFERENCES pipelines(id),
  workflow_id uuid REFERENCES workflows(id),
  repository_id uuid REFERENCES repositories(id),
  project_id uuid REFERENCES projects(id),
  device_id uuid REFERENCES devices(id),
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
