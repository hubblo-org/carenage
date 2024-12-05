import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { ProjectMetadata } from "$lib/types/carenage";

export const load: PageServerLoad = async ({ params }) => {
  const project: ProjectMetadata = await _fetchProject(params.id);
  if (!project) {
    error(404, "Not found");
  }
  return { project };
};

export async function _fetchProject(projectId: string) {
  try {
    const response = await fetch(`https://api.carenage.hubblo.org/projects/${projectId}`);
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }
    const project = await response.json();
    return project;
  } catch (err) {
    console.log(err.message);
  }
}
