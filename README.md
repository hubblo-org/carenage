# carenage

*Carenage* is a continuous integration tool meant to make environmental evaluation easier for software projects. Used in CI scripts, it queries needed metrics and metadata during the execution of a CI job to insert them into a database. Afterwards, the data is available as JSON objects through requests to a RESTful API and visualisable in a dashboard.

*Carenage* (*careening* in English) is a series of operations made on the hull of a ship in order to inspect, clean or repair it. Environmental evaluation can be difficult to put into place for software development teams. With data recovered during tests or benchmarks, *Carenage* aims to simplify this and allow for better evaluation and better application of eco-design practices.

## Development

### Backend

The `carenage` project is divided into several packages, with three binaries (`api`, `carenaged`, `carenage-cli`) and one library (`database`).

- `carenaged` contains the daemon program that inserts metadata and metrics into the database.
- `carenage-cli` contains the command-line interface program that can start and stop the execution of the daemon inside scripts.
- `api` contains the web server that receives requests and sends responses through a RESTful API.

The `database` library contains all methods needed to communicate with the database, most data structures used throughout the project, and modules to communicate with other external services (most notably [`boagent`](https://github.com/boavizta/boagent])).

To create environmental impacts metrics, `carenage` queries `boagent` to receive data on the hardware for the computer running processes. This data is made of environmental impact metrics (through queries to [boaviztapi](https://github.com/boavizta/boaviztapi)), and energy consumption metrics (through [scaphandre](https://github.com/hubblo-org/scaphandre)). `carenage` queries information on the computer configuration, then, throughout a CI script execution, queries information for each process running on the computer. All these pieces of information are then inserted into the database with a timestamp.


#### Building the project

With `rust` and `cargo` installed:

```
cd carenage
make build
```

#### Testing

Unit and integrations tests are implemented for the different packages of the project:

```
cd carenage
make test
``` 
