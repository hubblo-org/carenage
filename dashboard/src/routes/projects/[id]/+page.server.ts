import type { PageServerLoad } from "./$types";
import type { ProjectMetadata } from "$lib/types/carenage";

export const load: PageServerLoad = async ({ params, cookies }) => {
  cookies.set("projectid", params.id, { path: "/" });
  return { project: await fetchProject(params.id) };
};

async function fetchProject(projectId: string) {
  const response = await fetch(`https://api.carenage.hubblo.org/projects/${projectId}`);
  const project = (await response.json()) as Promise<ProjectMetadata>;

  return project;
}
