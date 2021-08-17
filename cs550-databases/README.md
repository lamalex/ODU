# CS 450 Project: Financial Management System

The team [`Slack` workspace](https://inigo-montoya.slack.com)

## Running the environment
Docker is the only required dependency is [Docker](https://www.docker.com/docker-community).
Docker is best installed using your systems relevant package manager. Someone has definitely written a guide to installing Docker on your system.
I used [Homebrew](https://brew.sh).


### Fully Dockerized env
from the project root
```
../cs450-project $ docker-compose up frontend # Runs the frontend and backend from inside of docker environments.
```
The backend is now running on port 8888, visit [http://localhost:8888/](http://localhost:8888/) to see `phpinfo` and verify the backend server is running.
All backend dependencies are installed on container start. If you need to add a new dependeny add it to `api/composer.json` and run `docker-compose restart` to install the new dependencies. If you have `composer` installed locally you can run `composer install`.

The frontend is running on port 8080, visit [http://localhost:8080](http://localhost:8080) and verify the registration page loads.
All frontend dependencies are installed on container start *inside of the container*. If you need to add a new dependeny add it to `frontend/package.json` and run `docker-compose stop; docker-compose start --build` to install the new dependencies.

**Start hacking**! You can modify frontend or backend files and they'll be live reloaded. Occasionally the frontend won't reload on save. Just save again and it should reload. Worst case you'll have to run `docker-compose restart`.

There's not a straight forward way to run the frontend or backend tests this way, but it's here if you need it. It saves you from needed to install things other than docker.

### Docker backend with local frontend dev server
#### Run the project backend
from the project root
```
../cs450-project $ docker-compose up
```
Runs the backend on port 8888. Same steps as above regarding backend apply.

For the frontend you'll need node v12 or greater installed.
From the `frontend/` directory
```
../cs450-project/frontend $ npm install # install frontend dependencies
../cs450-project/frontend $ npm run serve # run frontend dev server
```

### Frontend Framework
Frontend is written in [Vue](https://www.vuejs.org).
[`vue-cli`](https://cli.vuejs.org) is recommended, but not required!

To run the frontend open a terminal inside of the `frontend/` directory and run
`$ npm run serve`

For help with Vue:
1. Check the [docs](https://vuejs.org/v2/guide/)
2. Ask Alex on Slack

(in that order)
