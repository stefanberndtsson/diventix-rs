#[allow(non_camel_case_types,dead_code)]
#[derive(Clone,Copy)]
pub enum Layer {
    Background = 0,
    LayerA = 1,
    LayerB = 2,
    LayerC = 3,
    LayerD = 4,
    LogoA = 5,
    LogoB = 6,
    FrameMask = 7,
}

#[allow(non_camel_case_types,dead_code)]
pub enum Action {
    OutputAdjust(Adjust),
    OutputPlace(u16, u16, u16, u16),
}

#[allow(non_camel_case_types,dead_code)]
pub enum Adjust {
    HPos(u16),
    VPos(u16),
    HSize(u16),
    VSize(u16),
}

fn adjust_output(layer: Layer, adj: Adjust) -> String {
    match adj {
        Adjust::HPos(hp) => format!("1,{},{}pH", layer as u8, hp),
        Adjust::VPos(vp) => format!("1,{},{}pV", layer as u8, vp),
        Adjust::HSize(hs) => format!("1,{},{}pW", layer as u8, hs),
        Adjust::VSize(vs) => format!("1,{},{}pS", layer as u8, vs),
    }
}

fn place_output(layer: Layer, (x, y, w, h): (u16, u16, u16, u16)) -> Vec<String> {
    let mut place = Vec::new();
    place.push(adjust_output(layer, Adjust::HPos(x+32768)));
    place.push(adjust_output(layer, Adjust::VPos(y+32768)));
    place.push(adjust_output(layer, Adjust::HSize(w)));
    place.push(adjust_output(layer, Adjust::VSize(h)));
    place
}

pub fn layer(layer: Layer, action: Action) -> Vec<String> {
    match action {
        Action::OutputAdjust(adj) => vec![adjust_output(layer, adj)],
        Action::OutputPlace(x, y, w, h) => place_output(layer, (x, y, w, h)),
    }
}
