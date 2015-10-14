extern crate mpd;

use std::net::ToSocketAddrs;
use std::thread;
use std::sync::{Mutex, Arc, MutexGuard};

use self::mpd::idle::Idle;

pub use self::mpd::{Song, Status, State};

pub enum MpdState {
    NotConnected,
    Connected(mpd::Status, Option<mpd::Song>),
    Error(String)
}

pub struct MpdSource {
    state: Arc<Mutex<MpdState>>
}

fn get_mpd_state(client: &mut mpd::Client) -> mpd::error::Result<MpdState> {
    let status = try!(client.status());
    let song = try!(client.currentsong());
    Ok(MpdState::Connected(status, song))
}

impl MpdSource {
    pub fn new<A: ToSocketAddrs + Send + 'static>(addr: A) -> MpdSource {
        let state = Arc::new(Mutex::new(MpdState::NotConnected));
        let stateref = state.clone();
        MpdSource::start_thread(addr, stateref);
        MpdSource {state: state}
    }

    fn start_thread<A: ToSocketAddrs + Send + 'static>(addr: A, state: Arc<Mutex<MpdState>>) {
        fn swap_data(state: &Arc<Mutex<MpdState>>, new: MpdState) {
            let mut guard = state.lock().unwrap();
            *guard = new;
        }
        thread::spawn(move || {
            loop {
                match mpd::Client::connect(&addr) {
                    Ok(mut client) => {
                        {
                            let mstate = get_mpd_state(&mut client)
                                .unwrap_or_else(|e| MpdState::Error(format!("{}", e)));
                            swap_data(&state, mstate);
                        }
                        while let Ok(_) = client.wait(&[mpd::Subsystem::Player]) {
                            let mstate = get_mpd_state(&mut client)
                                .unwrap_or_else(|e| MpdState::Error(format!("{}", e)));
                            swap_data(&state, mstate);
                        }
                    },
                    Err(e) => {
                        let s = format!("{}", e);
                        swap_data(&state, MpdState::Error(s));
                    }
                };
                thread::sleep_ms(2000);
            }
        });
    }

    pub fn get(&self) -> MutexGuard<MpdState> {
        self.state.lock().unwrap()
    }
}
