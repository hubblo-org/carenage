CREATE TABLE projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) UNIQUE,
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ

);
CREATE TABLE workflows (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE pipelines (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE runs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE jobs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE tasks (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE containers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  start_date TIMESTAMPTZ,
  stop_date TIMESTAMPTZ
);

CREATE TABLE processes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  container_id UUID REFERENCES containers(id),
  pid INTEGER,
  exe VARCHAR(255),
  cmdline TEXT,
  state VARCHAR(255),
  start_date TIMESTAMPTZ DEFAULT localtimestamp,
  stop_date TIMESTAMPTZ
);

CREATE TABLE devices (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255),
  lifetime INTEGER,
  location CHARACTER(3)
);

CREATE TABLE components (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  device_id UUID REFERENCES devices(id),
  name VARCHAR(255),
  model VARCHAR(255),
  manufacturer VARCHAR(255)
);

CREATE TABLE component_characteristic (
  component_id UUID REFERENCES components(id),
  name VARCHAR(255),
  value VARCHAR(255)
);

CREATE TYPE event_type AS ENUM ('regular', 'custom', 'start', 'stop');

CREATE TABLE events (
  id UUID DEFAULT gen_random_uuid() UNIQUE,
  timestamp TIMESTAMPTZ,
  process_id UUID REFERENCES processes(id),
  task_id UUID REFERENCES tasks(id),
  job_id UUID REFERENCES jobs(id),
  run_id UUID REFERENCES runs(id),
  pipeline_id UUID REFERENCES pipelines(id),
  workflow_id UUID REFERENCES workflows(id),
  project_id UUID REFERENCES projects(id),
  device_id UUID REFERENCES devices(id),
  event_type event_type,
  user_label TEXT,
  CONSTRAINT primary_keys PRIMARY KEY (id, task_id, job_id, run_id, pipeline_id, workflow_id, project_id, device_id, process_id)
);

CREATE TABLE metrics (
  event_id UUID REFERENCES events(id),
  metric VARCHAR(255),
  value FLOAT8  
);
