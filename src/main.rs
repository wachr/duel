extern crate duel;
extern crate rusty_sword_arena;

use duel::{audio_loop, parse_args};
use rusty_sword_arena::game::*;
use rusty_sword_arena::gfx::Window;
use rusty_sword_arena::net::ServerConnection;
use rusty_sword_arena::VERSION;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread::Builder;
use std::time::Instant;

fn main() {
    let (name, host) = parse_args();
    println!("Using rusty_sword_arena v{}, name: {}, host: {}",
             VERSION, name, host);
    // Turbo fish operator ( `::<>` ) for specifying a type for a generic function.
    let (audio_tx, audio_rx) = channel::<&'static str>();
    let audio_handle = Builder::new()
        .name("Audio System".to_string())
        .spawn(move || audio_loop(audio_rx))
        .unwrap();

    let mut server_conn = ServerConnection::new(&host);
    let my_id = server_conn.join(&name);
    if my_id == 0 {
        println!("Either name is taken or server is full. Give it another try.");
        std::process::exit(3)
    }
    println!("My id is {}", my_id);

    let game_setting = server_conn.get_game_setting();
    println!("Client v{}, Server v{}", VERSION, game_setting.version);
    if VERSION != game_setting.version { std::process::exit(4) }

    let _ = audio_tx.send("startup");

    let mut players: HashMap<u8, Player> = HashMap::new();
    let mut my_input = PlayerInput::new();
    my_input.id = my_id;
    let mut mouse_pos = Vector2::new();
    let mut window = Window::new(None);
    let mut last_input_sent = Instant::now();
    let send_input_duration = std::time::Duration::new(15, 0);

    'game_loop: loop {
        for game_state in server_conn.poll_game_states() {
            // Remove any players who left
            players.retain(|k, v| game_state.player_states.contains_key(k));
            for (id, player_state) in game_state.player_states {
                if players.contains_key(&id) {
                    players.get_mut(&id).unwrap().update_state(player_state);
                } else {
                    players.insert(id, Player::new(player_state));
                }
            }
        }
        std::thread::sleep_ms(50);
        println!("players: {:?}", players);
        break 'game_loop;
    }

    // shutdown
    if server_conn.leave(my_id) {
        println!("Server acknowledges leaving");
    } else {
        println!("Server must have kicked us");
    }
    let _ = audio_tx.send("quit");
    let _ = audio_handle.join();
}

#[derive(Debug)]
struct Player {
    state: PlayerState,
}

impl Player {
    fn new(state: PlayerState) -> Self {
        Self {
            state,
        }
    }
    fn update_state(&mut self, state: PlayerState) {
        self.state = state;
    }
}
