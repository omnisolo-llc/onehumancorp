FROM node:20-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

COPY package.json package-lock.json* ./
RUN npm install --legacy-peer-deps --only=production

COPY .next ./.next
COPY public ./public

EXPOSE 3000

CMD ["npx", "next", "start", "-p", "3000"]
