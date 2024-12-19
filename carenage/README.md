# Carenage back-end

## Development

### Back-end

The `carenage` project is divided into several packages, with three binaries (`api`, `carenaged`, `carenage-cli`) and one library (`database`).

- `carenaged` contains the daemon program that inserts metadata and metrics into the database.
- `carenage-cli` contains the command-line interface program that can start and stop the execution of the daemon inside scripts.
- `api` contains the web server that receives requests and sends responses through a RESTful API.

The `database` library contains all methods needed to communicate with the database, most data structures used throughout the project, and modules to communicate with other external services (most notably [`boagent`](https://github.com/boavizta/boagent])).

To create environmental impacts metrics, `carenage` queries `boagent` to receive data on the hardware for the computer running processes. This data is made of environmental impact metrics (through queries to [boaviztapi](https://github.com/boavizta/boaviztapi)), and energy consumption metrics (through [scaphandre](https://github.com/hubblo-org/scaphandre)). `carenage` queries information on the computer configuration, then, throughout a CI script execution, queries information for each process running on the computer. All these pieces of information are then inserted into the database with a timestamp.


#### Building the project

With `rust` and `cargo` installed, you can run `make build_debug`.

#### Testing

Unit and integrations tests are implemented for the different packages of the project and can be executed with `make test`.


Some tests need a Postgres database set up to apply migrations and fixtures. If you have Docker installed, `make compose_dev` will set up the development / testing environment.
