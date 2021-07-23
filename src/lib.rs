// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//
// uptic - easy-to-use uptime crate for linux.
//
// there's Default implemented for Uptic structure, you can use it like this:
//
// let uptime = Uptic::default();

use std::io::{BufRead};

pub struct Uptic {
    pub days   : u64,
    pub hours  : u8,  // < 24
    pub minutes: u8,  // < 60
    pub raw_ss : u64
}

impl Default for Uptic {
    fn default() -> Self {
        let mut data = Uptic {
            days   : 0,
            hours  : 0,
            minutes: 0,
            raw_ss : 0
        }; data.init();

        data
    }
}

impl Uptic {
    fn read_lines<P>(&self, file: &P) -> std::io::Result<
        std::io::Lines<std::io::BufReader<std::fs::File>>
    > where P: AsRef<std::path::Path>, {
        Ok(std::io::BufReader::new(
            std::fs::File::open(file)?).lines())
    }

    #[cfg(not(target_os = "windows"))]
    fn init(&mut self) {
        if std::path::Path::new("/proc/uptime").exists() {
            if let Ok(lines) = self.read_lines(&"/proc/uptime") {
                for line in lines {
                    if let Ok(__line__) = line {
                        self.raw_ss = __line__.split(' ').next().unwrap().parse::<f64>().unwrap() as u64;
                    }
                }

                self.days    = (self.raw_ss / 60 / 60 / 24) ^ 0;
                self.hours   = ((self.raw_ss / 60 / 60 % 24) ^ 0) as u8;
                self.minutes = ((self.raw_ss / 60      % 60) ^ 0) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Uptic};

    #[test]
    fn hmm() {
        let uptime = Uptic::default();

        println!("d/s({}), h/s({}), m/s({})", uptime.days, uptime.hours, uptime.minutes);
    }
}
