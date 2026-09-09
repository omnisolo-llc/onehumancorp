FROM --platform=$BUILDPLATFORM node:22-alpine AS builder

WORKDIR /app

COPY src/ui/next/package.json src/ui/next/package-lock.json ./
RUN npm ci

COPY src/ui/next/ ./

ENV NEXT_TELEMETRY_DISABLED=1
RUN npm run build

FROM node:22-alpine

ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    HOSTNAME=0.0.0.0 \
    PORT=3000

WORKDIR /app

COPY --from=builder /app/package.json /app/package-lock.json ./
RUN npm ci

COPY --from=builder --chown=node:node /app/.next ./.next
COPY --from=builder --chown=node:node /app/public ./public
COPY --from=builder --chown=node:node /app/next.config.mjs ./next.config.mjs

USER node

EXPOSE 3000

CMD ["npm", "run", "start"]
