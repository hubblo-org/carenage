export async function fetchProject(projectId: string) {
  try {
    const response = await fetch(`https://api.carenage.hubblo.org/projects/${projectId}`);
    if (!response.ok) {
      throw new Error(
        `Failed to fetch project data with ID "${projectId}". Response status: ${response.status}`
      );
    }
    const project = await response.json();
    return project;
  } catch (err) {
    console.log(err.message);
  }
}

export async function fetchPipeline(pipelineId: string) {
  try {
    const response = await fetch(`https://api.carenage.hubblo.org/pipelines/${pipelineId}`);
    if (!response.ok) {
      throw new Error(
        `Failed to fetch pipeline data with ID "${pipelineId}". Response status: ${response.status}`
      );
    }
    const pipeline = await response.json();

    return pipeline;
  } catch (err) {
    console.log(err.message);
  }
}

export async function fetchRun(runId: string) {
  try {
    const response = await fetch(`https://api.carenage.hubblo.org/runs/${runId}`);
    if (!response.ok) {
      throw new Error(
        `Failed to fetch run data with ID "${runId}". Response status: ${response.status}`
      );
    }
    const run = await response.json();

    return run;
  } catch (err) {
    console.log(err.message);
  }
}
