# carenage


## Development

Unit tests in `/carenage/src/database.rs` depend on `sqlx`'s connection to a Postgres database. To allow it, add a `.env` file in the `/carenage`
folder with the following environment variable: `DATABASE_URL=<postgres://USER:PASSWORD@HOST:PORT>`, replacing the uppercase words with your
configuration details.
The `docker-compose.yml` sets up a Postgres database with the following URI : `postgres://carenage:password@localhost:5432`.
