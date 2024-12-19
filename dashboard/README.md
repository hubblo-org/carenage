# Carenage dashboard

This is the front-end interface for Carenage. It is a [SvelteKit](https://svelte.dev) project, thought as a dashboard representing metadata and environmental impact metrics for a given project, a CI pipeline, a CI run for a job, etc.

## Development

### Setup

With `npm` installed, you can do `make install` to setup needed dependencies for the project. Then, with `make run_dev`, this will launch the server for
the dashboard.

### Testing

Unit and integration tests are executed with `make component_test`, end-to-end testing with `make e2e_test`.

For integration testing and mocking responses from external services like `Boagent`, [msw](https://mswjs.io/) is used. You can add handlers for specific
URLs in `src/mocks/handlers.ts`. In a development environment, those will be able to handle fetching valid URLs by the server code, and render data on the adequate pages. Handlers work either in the browser and in a Node environment.
