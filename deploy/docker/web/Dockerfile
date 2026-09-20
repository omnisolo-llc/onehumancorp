# syntax=docker/dockerfile:1.7
# Use the repository root as context so the tracing root and locked dependency
# layout match native CI. Build on the target platform for native Node modules.
FROM node:22.22.1-bookworm-slim AS builder
WORKDIR /src
COPY package.json package-lock.json ./
COPY src/ui/next/package.json src/ui/next/package-lock.json ./src/ui/next/
RUN npm ci --no-audit --no-fund && npm --prefix src/ui/next ci --no-audit --no-fund
COPY src/ui/next ./src/ui/next
COPY .node-version ./
COPY scripts/build-web.mjs scripts/package-web.mjs scripts/native-web-entry.cjs ./scripts/
ENV NEXT_TELEMETRY_DISABLED=1
RUN npm run build:web

FROM node:22.22.1-bookworm-slim AS runtime
ENV NODE_ENV=production NEXT_TELEMETRY_DISABLED=1 HOSTNAME=0.0.0.0 PORT=3000
WORKDIR /app
COPY --from=builder --chown=node:node /src/target/native-web ./
USER node
EXPOSE 3000
CMD ["node", "src/ui/next/server.js"]
