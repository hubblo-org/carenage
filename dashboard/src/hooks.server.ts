import { dev } from "$app/environment";
import type { Handle } from "@sveltejs/kit";
import { isUUID } from "$lib/utils";

if (dev) {
  const { server } = await import("./mocks/node");
  server.listen();
}

export const handle: Handle = async ({ event, resolve }) => {
  if (!event.cookies.get("projectid")) {
    if (event.url.pathname.startsWith("/projects")) {
      const splitPathName = event.url.pathname.split("/");
      const projectId = splitPathName[2];
      if (isUUID(projectId)) {
        event.cookies.set("projectid", projectId, { path: "/" });
      }
    }
  }
  const response = await resolve(event);
  return response;
};
