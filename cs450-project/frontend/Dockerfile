FROM node:15

RUN apt-get -y update && apt-get install -y git
RUN npm install -g @vue/cli

WORKDIR /usr/src/app/frontend/
COPY package*.json ./
RUN npm install

EXPOSE 8080
USER node
CMD ["npm", "run", "serve"]
