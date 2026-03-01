# dependencies (package files) #################
FROM node:24-slim AS deps
WORKDIR /app

RUN corepack enable

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./

RUN pnpm install --frozen-lockfile

# build (ts files in src) #################
FROM node:24-slim AS build
WORKDIR /app

RUN corepack enable

COPY --from=deps /app/node_modules ./node_modules
COPY . .

RUN pnpm run build

# runtime (create image) #################
FROM node:24-slim AS runtime
WORKDIR /app

ENV NODE_ENV=production
ENV npm_config_production=true

RUN corepack enable

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./

RUN pnpm install --prod --frozen-lockfile

COPY --from=build /app/dist ./dist
RUN mkdir -p /app/cache && chown -R node:node /app

USER node
EXPOSE 3000
CMD ["node", "dist/server.js"]