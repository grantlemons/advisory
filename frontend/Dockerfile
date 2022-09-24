FROM node as builder
WORKDIR /app
COPY . .
RUN yarn install
RUN yarn run build

# -------

FROM nginx:1.23.1-alpine
COPY --from=builder /app/build /usr/share/nginx/html
