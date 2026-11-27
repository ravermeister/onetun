FROM debian:stable-slim
ARG TARGETARCH

RUN apt-get update \
    && apt-get install -y dumb-init \
    && rm -rf /var/lib/apt/lists/*

COPY ci/assets/ /tmp/assets/
RUN case "${TARGETARCH}" in \
      amd64) cp /tmp/assets/tunone-linux-amd64 /usr/local/bin/tunone ;; \
      arm64) cp /tmp/assets/tunone-linux-arm64 /usr/local/bin/tunone ;; \
      arm)   cp /tmp/assets/tunone-linux-armv7 /usr/local/bin/tunone ;; \
      *) echo "Unsupported architecture: ${TARGETARCH}" >&2; exit 1 ;; \
    esac \
    && chmod 0755 /usr/local/bin/tunone \
    && rm -rf /tmp/assets

USER 1000
ENTRYPOINT ["dumb-init", "/usr/local/bin/tunone"]
