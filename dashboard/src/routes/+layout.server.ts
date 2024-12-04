import type { LayoutServerLoad } from "./$types";
import type { ProjectMetadata } from "$lib/types/carenage";

export const load: LayoutServerLoad = async ({ cookies }) => {
  const projectId = cookies.get("projectid");
  return { project: await fetchProject(projectId) };
};

async function fetchProject(projectId: string) {
  const response = await fetch(`https://api.carenage.hubblo.org/projects/${projectId}`);
  const project = (await response.json()) as Promise<ProjectMetadata>;

  return project;
}
