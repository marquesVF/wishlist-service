Keep in mind that the data is not persisted in the container you run on Docker.

If you want to run the container with persisted data, first you need to create a container volume:

`docker volume create postgres_data`

Then you can run the container passing the folder path:

`postgres_data:your/persitent/path`.

## Setup

- Build the docker image
- Create a Docker image passing the postgres user password: `POSTGRES_PASSWORD=YOUR_PASSWORD docker build .`
