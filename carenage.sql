CREATE DATABASE carenage;

CREATE TABLE project (
  project_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE repository (
  repository_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE workflow (
  workflow_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE pipeline (
  pipeline_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE run (
  run_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE job (
  job_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE task (
  task_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE container (
  container_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE process (
  process_id integer PRIMARY KEY,
  container_id uuid FOREIGN KEY REFERENCES(container),
  exe VARCHAR(255),
  cmdline TEXT,
  state VARCHAR(255),
  start_date TIMESTAMP,
  stop_date TIMESTAMP,
);

CREATE TABLE machine (
  machine_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  lifetime INTEGER,
  location CHARACTER(3),
);

CREATE TABLE component (
  component_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  machine_id uuid FOREIGN KEY REFERENCES(machine),
  name VARCHAR(255),
  model VARCHAR(255),
  manufacturer VARCHAR(255),
);

CREATE TABLE component_characteristic (
  component_id uuid FOREIGN KEY REFERENCES(component),
  name VARCHAR(255),
  value VARCHAR(255),
);

CREATE TABLE event (
  timestamp TIMESTAMP,
  process_id uuid FOREIGN KEY REFERENCES (process),
  task_id uuid FOREIGN KEY REFERENCES (task),
  job_id uuid FOREIGN KEY REFERENCES (job),
  run_id uuid FOREIGN KEY REFERENCES (run),
  pipeline_id uuid FOREIGN KEY REFERENCES (pipeline),
  workflow_id uuid FOREIGN KEY REFERENCES (workflow),
  repository_id uuid FOREIGN KEY REFERENCES (repository),
  project_id uuid FOREIGN KEY REFERENCES (project),
  machine_id uuid FOREIGN KEY REFERENCES (machine),
  type VARCHAR(255),
  CPU_time_percent FLOAT,
  RAM_virt_bytes INTEGER,
  RAM_phy_bytes INTEGER,
  DISK_write_bytes INTEGER,
  DISK_read_bytes INTEGER,
  Process_consumption FLOAT,
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
  CONSTRAINT primary_keys PRIMARY KEY (process_id, task_id, job_id, pipeline_id, workflow_id, repository_id, project_id, machine_id),
);
