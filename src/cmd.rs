use crate::preset;
use crate::layer;
use crate::output;
use std::io::prelude::*;
use std::net::TcpStream;
use std::{thread,time};

#[derive(Debug)]
enum Command {
    DiVentix(String),
    DelayMS(u64),
}

#[derive(Debug)]
pub struct Cmd {
    cmds: Vec<Command>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            cmds: Vec::new(),
        }
    }

    fn sleep_delay(delay: u64) {
        if delay > 0 {
            let ms = time::Duration::from_millis(delay);
            thread::sleep(ms);
        }
    }
    
    pub fn delay(mut self, delay: u64) -> Self {
        self.cmds.push(Command::DelayMS(delay));
        self
    }
    
    pub fn preset(mut self, layout: preset::Layout) -> Self {
        for cmd in preset::layout(layout).drain(..) {
            self.cmds.push(Command::DiVentix(cmd));
        }
        self
    }

    pub fn layer(mut self, layer: layer::Layer, action: layer::Action) -> Self {
        for cmd in layer::layer(layer, action).drain(..) {
            self.cmds.push(Command::DiVentix(cmd));
        }
        self
    }

    pub fn output(mut self, port: output::Port, action: output::Action) -> Self {
        for cmd in output::output(port, action).drain(..) {
            self.cmds.push(Command::DiVentix(cmd));
        }
        self
    }

    fn write(conn: &mut TcpStream, cmd: String) -> Result<(), std::io::Error> {
        conn.write(cmd.as_bytes())?;
        let mut buf = vec![0u8; 128];
        let response = conn.read(&mut buf)?;
        println!("Response: {:?}: {}", response, String::from_utf8_lossy(&buf));
        Ok(())
    }
    
    pub fn send(&mut self, conn: &mut TcpStream) -> Result<(), std::io::Error> {
        for cmd in self.cmds.drain(..) {
            match cmd {
                Command::DiVentix(c) => { Cmd::write(conn, c)?; },
                Command::DelayMS(d) => { Cmd::sleep_delay(d); },
            }
        }
        Ok(())
    }
}
