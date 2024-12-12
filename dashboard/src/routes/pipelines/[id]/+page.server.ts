import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { fetchPipeline } from "$lib/fetchCarenage";

export const load: PageServerLoad = async ({ params }) => {
  const pipeline = await fetchPipeline(params.id);
  if (!pipeline) {
    error(404, "Not found");
  }
  return { pipeline: pipeline };
};
