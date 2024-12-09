import type { PageServerLoad } from "./$types";
import { fetchRun } from "$lib/fetchCarenage";
import { error } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ params }) => {
  const run = await fetchRun(params.id);
  if (!run) {
    error(404, "Not found");
  }
  return { run: await fetchRun(params.id) };
};
