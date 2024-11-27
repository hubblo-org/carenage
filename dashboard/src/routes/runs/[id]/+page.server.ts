import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, params }) => {
  const response = await fetch(`https://api.carenage.boukin.org/${params.id}`);
  const run = await response.json();

  return { run };
};
