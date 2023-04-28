# Docker file for webui service based on qwik framework

FROM node:16 as dev
RUN mkdir /app
WORKDIR /app
COPY ./src ./src
COPY ./package.json ./package.json
COPY ./tsconfig.json ./tsconfig.json
COPY ./vite.config.ts ./vite.config.ts
COPY ./package-lock.json ./package-lock.json
RUN npm install
EXPOSE 5173
CMD ["npm", "run", "dev"]

FROM node:12.16.1-alpine3.11 as build
COPY . /app
WORKDIR /app
RUN npm install
RUN npm run build

FROM nginx:1.17.9-alpine as prod
COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
