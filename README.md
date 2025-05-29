# UDP Hole
Small server for ["UDP hole punching"](https://en.wikipedia.org/wiki/UDP_hole_punching) to traverse NAT between clients.

Clients send a message `GAME###{game_id}` and get back corresponding player's `ip:port` for UDP traffic
