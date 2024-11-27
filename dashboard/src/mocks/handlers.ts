import { http, HttpResponse } from "msw";
import run from "../tests/fixtures/run.json";

export const handlers = [
  http.get("https://api.carenage.hubblo.org/runs/c86fe1e5-8828-4f53-822b-df7e2ced37db", () => {
    return HttpResponse.json({
      project_name: "hubblo/carenage",
      pipeline_id: 1520057997,
      run_id: 8228228299,
      job_name: "test_for_merge_request",
      job_status: "success",
      job_duration: 180,
      processes: run.processes
    });
  })
];
