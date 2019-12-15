mod cmd;
mod preset;
mod layer;

use std::io::prelude::*;
use std::net::TcpStream;
use std::env;


fn main() {
    let mut cmd = cmd::Cmd::new()
        .preset(preset::Layout::PIP4_In)
        .delay(2000)
        .layer(layer::Layer::LayerA, layer::Action::OutputAdjust(layer::Adjust::VSize(720))) 
        .layer(layer::Layer::LayerA, layer::Action::OutputAdjust(layer::Adjust::HSize(1920)))
        .delay(2000)
        .layer(layer::Layer::LayerB, layer::Action::OutputPlace(480, 0, 960, 540))
        ;
    dbg!(&cmd);
    let mut conn = TcpStream::connect("10.17.42.161:10500").unwrap();

    cmd.send(&mut conn).unwrap();
    
    //    preset_layout(&mut conn, PresetLayout::PIP4_In);
/*    
    let mut args = env::args();
    let _ = args.next();
    
    if let Some(mut arg) = args.next() {
        arg += "\r\n";
        println!("Sending: {:?}", arg);
        conn.write(arg.as_bytes());
    }
  */  
}
