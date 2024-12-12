import { error } from "@sveltejs/kit";
import { fetchProject } from "$lib/fetchCarenage";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies }) => {
  const projectId = cookies.get("projectid");
  const project = await fetchProject(projectId);
  if (!project) {
    error(404, "Not found");
  }

  return { project: project };
};
