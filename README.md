# UDP Hole
Small server for ["UDP hole punching"](https://en.wikipedia.org/wiki/UDP_hole_punching) to traverse NAT between clients.

Clients send a message `GAME###{game_id}` and get back corresponding player's `ip:port` for UDP traffic

## Setup

Start the UDP hole image on a server with a public ip

```sh
docker run -d -p 3400:3400/udp ghcr.io/finafisken/udp-hole:latest
```

## Test it

Run in the terminal of two different network devices, replace X.X.X.X with servers public ip

```sh
echo -n "GAME###abc123" | nc -u X.X.X.X 3400
```