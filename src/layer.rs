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
#[derive(Clone,Copy)]
pub enum LayerType {
    Layer,
    Background,
    Logo,
}

#[allow(non_camel_case_types,dead_code)]
#[derive(Clone,Copy)]
pub enum InputSource {
    None,
    Input(u8),
    Frame(u8),
    Logo(u8),
}

#[allow(non_camel_case_types,dead_code)]
pub enum Action {
    OutputAdjust(Adjust),
    OutputPlace(i32, i32, u16, u16),
    LayerSource(InputSource),
}

#[allow(non_camel_case_types,dead_code)]
pub enum Adjust {
    HPos(u16),
    VPos(u16),
    HSize(u16),
    VSize(u16),
    Alpha(u8),
}

fn layer_type(layer: Layer) -> LayerType {
    match layer {
        Layer::Background => LayerType::Background,
        Layer::FrameMask => LayerType::Background,
        Layer::LayerA | Layer::LayerB | Layer::LayerC | Layer::LayerD => {
            LayerType::Layer
        },
        Layer::LogoA | Layer::LogoB => LayerType::Logo
    }
}

fn adjust_output(layer: Layer, adj: Adjust) -> String {
    match adj {
        Adjust::HPos(hp) => format!("1,{},{}pH", layer as u8, hp),
        Adjust::VPos(vp) => format!("1,{},{}pV", layer as u8, vp),
        Adjust::HSize(hs) => format!("1,{},{}pW", layer as u8, hs),
        Adjust::VSize(vs) => format!("1,{},{}pS", layer as u8, vs),
        Adjust::Alpha(a) => format!("1,{},{}pA", layer as u8, a),
    }
}

fn place_output(layer: Layer, (x, y, w, h): (i32, i32, u16, u16)) -> Vec<String> {
    let mut place = Vec::new();
    place.push(adjust_output(layer, Adjust::HPos((x+32768) as u16)));
    place.push(adjust_output(layer, Adjust::VPos((y+32768) as u16)));
    place.push(adjust_output(layer, Adjust::HSize(w)));
    place.push(adjust_output(layer, Adjust::VSize(h)));
    place
}

fn layer_source(layer: Layer, source: InputSource) -> String {
    let source_num = match source {
        InputSource::None => 0,
        InputSource::Frame(i) => i,
        InputSource::Input(i) => i,
        InputSource::Logo(i) => i,
    };
    match layer_type(layer) {
        LayerType::Layer | LayerType::Background | LayerType::Logo => {
            format!("1,{},{}IN", layer as u8, source_num)
        }
    }
}

pub fn layer(layer: Layer, action: Action) -> Vec<String> {
    match action {
        Action::OutputAdjust(adj) => vec![adjust_output(layer, adj)],
        Action::OutputPlace(x, y, w, h) => place_output(layer, (x, y, w, h)),
        Action::LayerSource(src) => vec![layer_source(layer, src)],
    }
}
