import { http, HttpResponse } from "msw";
import run from "../tests/fixtures/run.json";
import project from "../tests/fixtures/project_metadata.json";
import pipeline from "../tests/fixtures/pipeline.json";

const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
const runId = "d910fcd3-fef1-4077-9294-efea1975e3fc";
const pipelineId = "d199e857-fb0f-46b1-9846-74e53b494740";
const carenageApiBaseUrl = "https://api.carenage.hubblo.org";

export const handlers = [
  http.get(`${carenageApiBaseUrl}/runs/${runId}`, () => {
    return HttpResponse.json(run);
  }),
  http.get(`${carenageApiBaseUrl}/projects/${projectId}`, () => {
    return HttpResponse.json(project);
  }),
  http.get(`${carenageApiBaseUrl}/projects/invalid-id`, () => {
    return new HttpResponse(null, {
      status: 404,
      statusText: "Not found"
    });
  }),
  http.get(`${carenageApiBaseUrl}/runs/invalid-id`, () => {
    return new HttpResponse(null, {
      status: 404,
      statusText: "Not found"
    });
  }),
  http.get(`${carenageApiBaseUrl}/pipelines/invalid-id`, () => {
    return new HttpResponse(null, {
      status: 404,
      statusText: "Not found"
    });
  }),
  http.get(`${carenageApiBaseUrl}/pipelines/${pipelineId}`, () => {
    return HttpResponse.json(pipeline);
  })
];
