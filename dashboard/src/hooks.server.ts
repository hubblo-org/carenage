import { dev } from "$app/environment";
import type { Handle } from "@sveltejs/kit";

if (dev) {
  const { server } = await import("./mocks/node");
  server.listen();
}

export const handle: Handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith("/projects")) {
    const splitPathName = event.url.pathname.split("/");
    const projectId = splitPathName[2];
    event.cookies.set("projectid", projectId, { path: "/" });
  }
  const response = await resolve(event);
  return response;
};
