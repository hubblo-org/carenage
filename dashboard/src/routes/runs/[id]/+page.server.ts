import type { PageServerLoad } from "./$types";
import type { CiRun } from "$lib/types/carenage";

export const load: PageServerLoad = async ({ params }) => {
  return { run: await fetchRun(params.id) };
};

async function fetchRun(run_id: string) {
  const response = await fetch(`https://api.carenage.hubblo.org/runs/${run_id}`);
  const run = (await response.json()) as Promise<CiRun>;

  return run;
}
