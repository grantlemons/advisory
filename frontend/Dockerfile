FROM node as builder
WORKDIR /app
COPY . .
RUN yarn install
RUN yarn run build

# -------

FROM node:18-slim
WORKDIR /app
COPY --from=builder /app/package.json .
COPY --from=builder /app/yarn.lock .
COPY --from=builder /app/build .
RUN yarn --prod
CMD ["node", "index.js"]
