export async function fetchProject(projectId: string) {
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
