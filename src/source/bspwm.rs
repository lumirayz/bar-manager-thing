use std::process;
use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::fmt;

#[derive(Copy, Clone)]
pub enum OccupiedMode { Urgent, Occupied, Free }

impl fmt::Display for OccupiedMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            OccupiedMode::Urgent => "Urgent",
            OccupiedMode::Occupied => "Occupied",
            OccupiedMode::Free => "Free"
        })
    }
}

pub struct Desktop {
    pub name: String,
    pub focus: bool,
    pub occupied: OccupiedMode
}

pub struct Monitor {
    pub name: String,
    pub focus: bool
}

pub struct BspwmData {
    pub monitors: Vec<Monitor>,
    pub desktops: Vec<Desktop>
}

pub struct BspwmSource {
    data: Arc<Mutex<BspwmData>>
}

impl BspwmSource {
    pub fn new() -> BspwmSource {
        let data = Arc::new(Mutex::new(BspwmData {
            monitors: vec![],
            desktops: vec![]
        }));
        let dataref = data.clone();
        thread::spawn(move || {
            let p = process::Command::new("bspc")
                .arg("control").arg("--subscribe")
                .stdout(process::Stdio::piped())
                .spawn()
                .unwrap();
            let mut buf: Vec<u8> = Vec::new();
            for c in p.stdout.unwrap().bytes() {
                let c = c.unwrap();
                if c == 0x0A {
                    let d = String::from_utf8(buf.clone()).unwrap();
                    buf = vec![];
                    let mut _data = dataref.lock().unwrap();
                    _data.update(d);
                }
                else {
                    buf.push(c);
                }
            }
        });
        BspwmSource {
            data: data
        }
    }

    pub fn get(&self) -> MutexGuard<BspwmData> {
        self.data.lock().unwrap()
    }
}

impl BspwmData {
    fn update(&mut self, v: String) {
        self.monitors = vec![];
        self.desktops = vec![];
        let mut lastmonfocus = false;
        for rec in v[1..].split(':') {
            let cmd = &rec[..1];
            let args = &rec[1..];
            match cmd {
                "m" => {
                    self.monitors.push(Monitor {
                        name: args.to_string(),
                        focus: false
                    });
                    lastmonfocus = false;
                },
                "M" => {
                    self.monitors.push(Monitor {
                        name: args.to_string(),
                        focus: true
                    });
                    lastmonfocus = true;
                },
                "o" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: false,
                        occupied: OccupiedMode::Occupied
                    });
                },
                "O" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: lastmonfocus,
                        occupied: OccupiedMode::Occupied
                    });
                },
                "f" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: false,
                        occupied: OccupiedMode::Free
                    });
                },
                "F" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: lastmonfocus,
                        occupied: OccupiedMode::Free
                    });
                },
                "u" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: false,
                        occupied: OccupiedMode::Urgent
                    });
                },
                "U" => {
                    self.desktops.push(Desktop {
                        name: args.to_string(),
                        focus: lastmonfocus,
                        occupied: OccupiedMode::Urgent
                    });
                },
                "L" => {
                    // TODO
                },
                _ => {}
            }
        }
    }
}
