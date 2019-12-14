#[allow(non_camel_case_types,dead_code)]
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

pub fn layer(layer: Layer, action: Action) -> String {
    match action {
        Action::OutputAdjust(adj) => adjust_output(layer, adj),
    }
}
