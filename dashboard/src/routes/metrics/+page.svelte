<script lang="ts">
  import Run from "$lib/run.svelte";
  import Header from "$lib/components/header.svelte";
  import type { CiRun, Process } from "../../types/carenage";

  import processes from "../../tests/fixtures/run.json";

  const processes_data: Array<Process> = processes.processes;

  const run: CiRun = {
    project_name: "hubblo/carenage",
    pipeline_id: "#1520057997",
    run_id: "#8228228299",
    job_name: "test_for_merge_request",
    job_status: "success",
    job_duration: 180,
    processes: processes_data
  };
</script>

<Header {run} />
<Run {run} />
