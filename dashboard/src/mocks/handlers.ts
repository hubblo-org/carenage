import { http, HttpResponse } from "msw";
import run from "../tests/fixtures/run.json";
import project from "../tests/fixtures/project_metadata.json";
import pipeline from "../tests/fixtures/pipeline.json";

export const handlers = [
  http.get("https://api.carenage.hubblo.org/runs/d910fcd3-fef1-4077-9294-efea1975e3fc", () => {
    return HttpResponse.json(run);
  }),
  http.get("https://api.carenage.hubblo.org/projects/3a1f9a71-fdd2-4e89-9769-70cfb731a02d", () => {
    return HttpResponse.json(project);
  }),
  http.get("https://api.carenage.hubblo.org/pipelines/d199e857-fb0f-46b1-9846-74e53b494740", () => {
    return HttpResponse.json(pipeline);
  })
];
