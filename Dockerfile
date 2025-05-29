FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# download bin and make it executable
RUN curl -L -o udp-hole https://github.com/finafisken/udp-hole/releases/download/v0.1.0/udp-hole && \
    chmod +x udp-hole

# UDP port 3400
EXPOSE 3400/udp

CMD ["./udp-hole"]
