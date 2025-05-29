FROM debian:bullseye-slim

WORKDIR /app

COPY udp-hole .

RUN chmod +x udp-hole

# UDP port 3400
EXPOSE 3400/udp

CMD ["./udp-hole"]
