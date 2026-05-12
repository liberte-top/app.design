# syntax=docker/dockerfile:1.7

FROM node:20-bookworm-slim AS builder
WORKDIR /app/app.design/web
RUN corepack enable
ARG VITE_ENV_LABEL=compose
ENV VITE_ENV_LABEL=$VITE_ENV_LABEL

COPY web/package.json web/pnpm-lock.yaml* ./
RUN --mount=type=cache,id=pnpm-store,target=/root/.local/share/pnpm/store \
    --mount=type=secret,id=liberte_packages_npm_token,required=true \
    set -eu; \
    trap 'rm -f /root/.npmrc' EXIT; \
    token="$(cat /run/secrets/liberte_packages_npm_token 2>/dev/null || true)"; \
    if [ -n "$token" ]; then \
      printf '%s\n%s\n' "@liberte:registry=https://packages.liberte.top/" "//packages.liberte.top/:_authToken=$token" > /root/.npmrc; \
    fi; \
    pnpm install --no-frozen-lockfile

COPY web ./
RUN pnpm build

FROM nginx:1.27-alpine AS runtime
COPY web/docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/app.design/web/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
