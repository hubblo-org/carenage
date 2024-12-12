import { dev } from "$app/environment";

if (dev) {
  const { worker } = await import("./mocks/browser");
  await worker.start({
    onUnhandledRequest(request, print) {
      // No warning on unhandled internal Svelte requests: no mocking needed.
      if (request.url.includes("svelte")) {
        return;
      }
      print.warning();
    }
  });
}
