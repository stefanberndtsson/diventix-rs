mod cmd;
mod preset;
mod layer;
mod output;

use std::io::prelude::*;
use std::net::TcpStream;
use std::env;


fn main() {
    let d1 = 200;
    let mut cmd = cmd::Cmd::new()
        .preset(preset::Layout::PIP4_In)
        .layer(layer::Layer::LayerA, layer::Action::LayerSource(layer::InputSource::Input(1)))
        .layer(layer::Layer::LayerB, layer::Action::LayerSource(layer::InputSource::Input(2)))
        .layer(layer::Layer::LayerD, layer::Action::LayerSource(layer::InputSource::Input(4)))
        .delay(2000)
        .layer(layer::Layer::LayerB, layer::Action::OutputPlace(320, 180, 1280, 720))
        .delay(1000)
        .layer(layer::Layer::LayerD, layer::Action::LayerSource(layer::InputSource::Input(1)))
        .delay(1000)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(255)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(230)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(210)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(190)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(170)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(150)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(130)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(110)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(90)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(70)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(50)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(30)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(10)))
        .delay(d1)
        .layer(layer::Layer::LayerD, layer::Action::OutputAdjust(layer::Adjust::Alpha(0)))
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
