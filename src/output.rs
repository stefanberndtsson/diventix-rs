#[allow(non_camel_case_types,dead_code)]
#[derive(Clone,Copy)]
pub enum Port {
    Main=0,
    Preview=1,
}

#[allow(non_camel_case_types,dead_code)]
#[derive(Clone,Copy)]
pub enum Format {
    PAL=0,
    NTSC=1,
    R_480p=2,
    R_576p=3,
    // ...
    R_1024x768=12,
    // ...
    R_1280x720=21,
    R_1920x1080=22,
    R_1920x1080Sharp=23,
    // ...
    R_1920x1080HDTV=27,
    R_1920x1080Sharp2=28,
    R_1366x768=29,
    R_1280x720HDTV=30,
}

#[allow(non_camel_case_types,dead_code)]
#[derive(Clone,Copy)]
pub enum Rate {
    Custom=0,
    Hz_23_97=1,
    Hz_24=2,
    Hz_25=3,
    Hz_29_97=4,
    Hz_30=5,
    Hz_50=6,
    Hz_59_94=7,
    Hz_60=8,
    Hz_72=9,
    Hz_75=10,
}



#[allow(non_camel_case_types,dead_code)]
pub enum Action {
    Rate(Rate),
    Format(Format),
    Output(Format,Rate),
}

fn output_format(port: Port, format: Format) -> String {
    format!("{},{}OF", port as u8, format as u8)
}

fn output_rate(port: Port, format: Rate) -> String {
    format!("{},{}OF", port as u8, format as u8)
}

fn output_full(port: Port, format: Format, rate: Rate) -> Vec<String> {
    vec![output_format(port, format), output_rate(port, rate)]
}

pub fn output(port: Port, action: Action) -> Vec<String> {
    match action {
        Action::Output(format,rate) => output_full(port, format, rate),
        Action::Format(format) => vec![output_format(port, format)],
        Action::Rate(rate) => vec![output_rate(port, rate)],
    }
}

