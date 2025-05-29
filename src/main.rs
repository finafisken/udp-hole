use std::{
    collections::{HashMap, HashSet},
    iter,
    net::{SocketAddr, UdpSocket},
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3400")?;

    let mut buf = [0u8; 1024];

    let mut game_index: HashMap<String, HashSet<SocketAddr>> = HashMap::new();

    loop {
        let (nr_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..nr_of_bytes]).into_owned();

        if let Some(game_id) = msg.trim().strip_prefix("GAME###") {
            game_index
                .entry(game_id.into())
                .and_modify(|player_set| {
                    player_set.insert(src_addr);
                })
                .or_insert(iter::once(src_addr).collect());

            let player_set = game_index
                .get_mut(game_id.into())
                .expect("No HashSet at key");

            if player_set.len() == 2 {
                let mut player_iter = player_set.iter();
                let p1_addr = player_iter.next().expect("Too few players!");
                let p2_addr = player_iter.next().expect("Too few players!");

                socket.send_to(format!("{}\n", p2_addr).as_bytes(), p1_addr)?;
                socket.send_to(format!("{}\n", p1_addr).as_bytes(), p2_addr)?;

                player_set.clear();
            }
        } else {
            println!("Unknown message from {}: {}", src_addr, msg);
        }
    }
}
