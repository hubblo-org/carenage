import { describe, it, expect } from "vitest";
import { fetchPipeline, fetchProject } from "$lib/fetchCarenage";
import type { CiPipelineMetadata, CiRunMetadata, ProjectMetadata } from "$lib/types/carenage";

const { server } = await import("../mocks/node");
server.listen();
describe("fetchProject test suite", () => {
  const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
  it("fetches data from the Carenage API with a valid project ID", async () => {
    const projectMetadata: ProjectMetadata = await fetchProject(projectId);
    expect(projectMetadata.project_name).toEqual("hubblo/carenage");
  });
});

describe("fetchPipeline test suite", () => {
  const pipelineId = "d199e857-fb0f-46b1-9846-74e53b494740";
  it("fetches data from the Carenage API with a valid pipeline ID", async () => {
    const pipelineMetadata: CiPipelineMetadata = await fetchPipeline(pipelineId);
    expect(pipelineMetadata.pipeline_repo_url).toEqual(
      "https://gitlab.com/hubblo/carenage/-/pipelines/1520057997"
    );
  });
});

describe("fetchRun test suite", () => {
  const runId = "d910fcd3-fef1-4077-9294-efea1975e3fc";
  it("fetches data from the Carenage API with a valid run ID", async () => {
    const runMetadata: CiRunMetadata = await fetchRun(runId);
    expect(runMetadata.run_repo_url).toEqual();
  });
});
