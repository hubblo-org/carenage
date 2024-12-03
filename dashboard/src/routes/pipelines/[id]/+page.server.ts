import type { PageServerLoad } from "./$types";
import type { CiPipelineMetadata } from "$lib/types/carenage";

export const load: PageServerLoad = async ({ params }) => {
  return { pipeline: await fetchPipeline(params.id) };
};

async function fetchPipeline(pipeline_id: string) {
  const response = await fetch(`https://api.carenage.hubblo.org/pipelines/${pipeline_id}`);
  const pipeline = (await response.json()) as Promise<CiPipelineMetadata>;

  return pipeline;
}
