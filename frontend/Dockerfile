FROM node AS build

WORKDIR /app

COPY package.json ./
COPY yarn.lock ./
RUN yarn install
COPY . ./
RUN yarn run build

FROM nginx:stable-alpine
COPY --from=build /app/public /usr/share/nginx/html

