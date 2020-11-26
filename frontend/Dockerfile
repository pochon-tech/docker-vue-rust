# 開発環境
FROM node:12.16.3-alpine as develop-stage
WORKDIR /app
COPY package*.json ./
# RUN rm -rf node_modules package-lock.json && yarn install
RUN yarn install
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN yarn build

# 本番環境
FROM nginx:1.17.10-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
