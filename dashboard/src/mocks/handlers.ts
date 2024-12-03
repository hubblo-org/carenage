import { http, HttpResponse } from "msw";
import run from "../tests/fixtures/run.json";
import pipeline from "../tests/fixtures/project_metadata.json";

export const handlers = [
  http.get("https://api.carenage.hubblo.org/runs/c86fe1e5-8828-4f53-822b-df7e2ced37db", () => {
    return HttpResponse.json(run);
  }),
  http.get("https://api.carenage.hubblo.org/projects/3a1f9a71-fdd2-4e89-9769-70cfb731a02d", () => {
    return HttpResponse.json(pipeline);
  })
];
