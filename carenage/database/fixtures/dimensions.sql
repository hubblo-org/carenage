CREATE OR REPLACE function public.project_ids()
	RETURNS uuid[] 
	LANGUAGE plpgsql
AS
$BODY$
DECLARE nowts TIMESTAMPTZ;
DECLARE p_id UUID;
DECLARE w_id UUID;
DECLARE pip_id UUID;
DECLARE j_id UUID;
DECLARE r_id UUID;
DECLARE t_id UUID;
DECLARE c_id UUID;
DECLARE proc_id UUID;
DECLARE d_id UUID;
BEGIN
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
	  start_date TIMESTAMPTZ,
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
	  project_id UUID REFERENCES projects(id),
	  workflow_id UUID REFERENCES workflows(id),
	  pipeline_id UUID REFERENCES pipelines(id),
	  job_id UUID REFERENCES jobs(id),
	  run_id UUID REFERENCES runs(id),
	  task_id UUID REFERENCES tasks(id),
	  process_id UUID REFERENCES processes(id),
	  device_id UUID REFERENCES devices(id),
	  event_type event_type,
	  user_label TEXT,
	  CONSTRAINT primary_keys PRIMARY KEY (id, project_id, workflow_id, pipeline_id, job_id, run_id, task_id, process_id, device_id)
	);

	CREATE TABLE metrics (
	  event_id UUID REFERENCES events(id),
	  metric VARCHAR(255),
	  value NUMERIC  
	);

	nowts := CURRENT_TIMESTAMP;
	INSERT INTO PROJECTS (name, start_date) VALUES ('my_web_app', nowts) RETURNING id INTO p_id;
	INSERT INTO WORKFLOWS (name, start_date)VALUES ('workflow_my_web_app', nowts) RETURNING id INTO w_id;
	INSERT INTO PIPELINES (name, start_date) VALUES ('Run tests on merge request', nowts) RETURNING id INTO pip_id;
	INSERT INTO JOBS (name, start_date) VALUES ('tests', nowts) RETURNING id INTO j_id;
	INSERT INTO RUNS (name, start_date) VALUES ('run_tests_01', nowts) RETURNING id INTO r_id;
	INSERT INTO TASKS (name, start_date) VALUES ('build_env_and_test', nowts) RETURNING id INTO t_id;
	INSERT INTO DEVICES (name, lifetime, location) VALUES ('dell r740', 5, 'FRA') RETURNING id INTO d_id;
	INSERT INTO COMPONENTS (device_id, name, model, manufacturer) VALUES (d_id, 'cpu',  'Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz', 'Intel Corp.') RETURNING id INTO c_id;
	INSERT INTO COMPONENT_CHARACTERISTIC (component_id, name, value) VALUES (c_id, 'core_units', 8); 
	INSERT INTO PROCESSES (pid, exe, cmdline, state, start_date) VALUES (4336, '/snap/firefox/4336/usr/lib/firefox/firefox', '/snap/firefox/4336/usr/lib/firefox/firefox-contentproc-childID58-isForBrowser-prefsLen32076-prefMapSize244787-jsInitLen231800-parentBuildID20240527194810-greomni/snap/firefox/4336/usr/lib/firefox/omni.ja-appomni/snap/firefox/4336/usr/lib/firefox/browser/omni.ja-appDir/snap/firefox/4336/usr/lib/firefox/browser{1e76e076-a55a-41cf-bf27-94855c01b247}3099truetab', 'running', nowts) RETURNING id INTO proc_id;
	RETURN ARRAY[p_id, w_id, pip_id, j_id, r_id, t_id, proc_id, d_id];
END
$BODY$;
