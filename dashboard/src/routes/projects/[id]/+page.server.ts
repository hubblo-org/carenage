import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { ProjectMetadata } from "$lib/types/carenage";
import { fetchProject } from "$lib/fetchCarenage";

export const load: PageServerLoad = async ({ params }) => {
  const project: ProjectMetadata = await fetchProject(params.id);
  if (!project) {
    error(404, "Not found");
  }
  return { project };
};
