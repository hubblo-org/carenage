import { describe, it, expect } from "vitest";
import { _fetchProject } from "../routes/projects/[id]/+page.server";
import type { ProjectMetadata } from "$lib/types/carenage";

const { server } = await import("../mocks/node");
server.listen();
describe("fetchProject test suite", () => {
  const projectId = "3a1f9a71-fdd2-4e89-9769-70cfb731a02d";
  it("fetches data from the Carenage API with a valid project ID", async () => {
    const projectMetadata: ProjectMetadata = await _fetchProject(projectId);
    expect(projectMetadata.project_name).toEqual("hubblo/carenage");
  });
});
