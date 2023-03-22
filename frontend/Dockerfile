FROM node as builder
WORKDIR /app
COPY ./package.json ./
COPY ./yarn.lock ./
COPY ./svelte.config.js ./
COPY ./src/theme/ ./src/theme
RUN yarn install
RUN mkdir ./static && yarn run gentheme
COPY . .
RUN yarn run vbuild --mode development

# -------

FROM nginx:1.23.1-alpine
COPY ./nginx-conf.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/build /usr/share/nginx/html
