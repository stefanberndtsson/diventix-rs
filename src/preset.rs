#[allow(non_camel_case_types,dead_code)]
pub enum Layout {
    None = 0,
    PIP4_In = 1,
    PIP4_Out = 2,
    PIP3_Vert_1_2 = 3,
    PIP3_Vert_2_1 = 4,
}

pub fn layout(layout: Layout) -> Vec<String> {
    vec![String::from(format!("{}pL", layout as u8))]
}
