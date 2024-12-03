import type { PageServerLoad } from "./$types";
import type { ProjectMetadata } from "$lib/types/carenage";

export const load: PageServerLoad = async ({ params }) => {
  return { project: await fetchProject(params.id) };
};

async function fetchProject(project_id: string) {
  const response = await fetch(`https://api.carenage.hubblo.org/projects/${project_id}`);
  const project = (await response.json()) as Promise<ProjectMetadata>;

  return project;
}
