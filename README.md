# carenage

*Carenage* is a continuous integration tool meant to make environmental evaluation easier for software projects. Used in CI scripts, it queries needed metrics and metadata during the execution of a CI job to insert them into a database. Afterwards, the data is available as JSON objects through requests to a RESTful API and visualisable in a dashboard.

*Carenage* (*careening* in English) is a series of operations made on the hull of a ship in order to inspect, clean or repair it. Environmental evaluation can be difficult to put into place for software development teams. With data recovered during tests or benchmarks, *Carenage* aims to simplify this and allow for better evaluation and better application of eco-design practices.

## Using Carenage

You can use Carenage in your continuous integration scripts, if the CI environment you will be using has the needed services already set up. The section [Setting up Carenage](#setting-up-carenage) gives a bit more information on that.
If that is the case, you can include the template available in `ci/templates` that downloads and sets up `carenage` for a Debian environment. This can be done with the following command in the `.gitlab-ci.yml` of your project:
 
```
include:
  - remote: "https://gitlab.com/hubblo/carenage/-/raw/main/ci/templates/setup-carenage.yml"
    inputs:
     lifetime: 6 
```

With `inputs`, some parameters used by `carenage` can be modified in place of the default ones, as documented in the template.

## Self-hosting Carenage 

`carenage` is meant to be executed in a continuous integration environment, such as a Gitlab runner. As it is dependent on several other services, namely [boaviztapi](https://github.com/boavizta/boaviztapi) and [scaphandre](https://github.com/hubblo-org/scaphandre) through [boagent](https://github.com/boavizta/boagent) and a PostgreSQL database, there is a bit of configuration to take care of if you want to self-host `carenage`.

### Gitlab runner

If you have set up your own [Gitlab runner](https://docs.gitlab.com/runner/install/index.html), you can add all needed services so that any CI pipeline can be executed without having to repeat each service setup process for each job.
In the folder `ci/docker`, you can find examples of a minimal `config.toml` for a Gitlab runner, and of a `docker-compose.yml` to see how to orchestrate all needed containers, with the associated network and volumes.
You may have to tweak the configuration for your own specific needs, but with this minimal configuration and orchestration set into place, a CI script similar to `ci/templates/setup-carenage.yml` should be able to use `carenage`.

## Development

### Back-end

The `carenage` project is divided into several packages, with three binaries (`api`, `carenaged`, `carenage-cli`) and one library (`database`).

- `carenaged` contains the daemon program that inserts metadata and metrics into the database.
- `carenage-cli` contains the command-line interface program that can start and stop the execution of the daemon inside scripts.
- `api` contains the web server that receives requests and sends responses through a RESTful API.

The `database` library contains all methods needed to communicate with the database, most data structures used throughout the project, and modules to communicate with other external services (most notably [`boagent`](https://github.com/boavizta/boagent)).

To create environmental impacts metrics, `carenage` queries `boagent` to receive data on the hardware for the computer running processes. This data is made of environmental impact metrics (through queries to [boaviztapi](https://github.com/boavizta/boaviztapi)), and energy consumption metrics (through [scaphandre](https://github.com/hubblo-org/scaphandre)). `carenage` queries information on the computer configuration, then, throughout a CI script execution, queries information for each process running on the computer. All these pieces of information are then inserted into the database with a timestamp.


#### Building the project

With `rust` and `cargo` installed:

```
cd carenage
make build_debug
```

#### Testing

Unit and integrations tests are implemented for the different packages of the project:

```
cd carenage
make test
``` 

Some tests need a Postgres database set up to apply migrations and fixtures. If you have Docker installed, `make compose_dev` will set up the development / testing environment.

### Front-end

#### Setup

With `npm` installed, you can do 

```
cd dashboard
make install
```

to setup needed dependencies for the project. Then, with `make run_dev`, this will launch the server for the dashboard.

#### Testing

Unit and integration tests are executed with 
```
cd dashboard
make component_test
```

end-to-end tests with 
```
cd dashboard
make e2e_test
```

For integration testing and mocking responses from external services like `Boagent`, [msw](https://mswjs.io/) is used. You can add handlers for specific
URLs in `src/mocks/handlers.ts`. In a development environment, those will be able to handle fetching valid URLs by the server code, and render data on the adequate pages. Handlers work either in the browser and in a Node environment.
