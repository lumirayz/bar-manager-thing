use std::process;
use std::io::Write;

use component::core::Component;

#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right
}

pub trait Bar {
    fn text(&mut self, &str);
    fn bgcol(&mut self, &str);
    fn fgcol(&mut self, &str);
    fn ucol(&mut self, &str);
    fn swapcol(&mut self);
    fn font(&mut self, u64);
    fn align(&mut self, Alignment);
    fn flush(&mut self);
}

pub struct LemonBar {
    process: process::Child
}

impl LemonBar {
    pub fn new() -> LemonBar {
        let p = process::Command::new("lemonbar")
            .arg("-f").arg("lemon:size=10")
            .arg("-f").arg("-wuncon-siji-medium-r-normal--10-100-75-75-c-80-iso10646-1")
            .arg("-g").arg("1840x20")
            .arg("-B").arg("#2D2B33")
            .arg("-F").arg("#FEFFFF")
            .stdin(process::Stdio::piped())
            .spawn()
            .unwrap();
        LemonBar {
            process: p
        }
    }

    pub fn write(&mut self, data: &str) {
        let mut p = self.process.stdin.as_mut().unwrap();
        p.write(data.as_bytes()).unwrap();
    }
}

impl Bar for LemonBar {
    fn text(&mut self, text: &str) {
        self.write(text);
    }

    fn bgcol(&mut self, col: &str) {
        self.write(&format!("%{{B{}}}", col));
    }

    fn fgcol(&mut self, col: &str) {
        self.write(&format!("%{{F{}}}", col));
    }
    
    fn ucol(&mut self, col: &str) {
        self.write(&format!("%{{U{}}}", col));
    }

    fn swapcol(&mut self) {
        self.write("%{R}");
    }

    fn font(&mut self, id: u64) {
        self.write(&format!("%{{T{}}}", id));
    }

    fn align(&mut self, alignment: Alignment) {
        self.write(&format!("%{{{}}}", match alignment {
            Alignment::Left => "l",
            Alignment::Center => "c",
            Alignment::Right => "r"
        }));
    }

    fn flush(&mut self) {
        self.write("\n");
    }
}
