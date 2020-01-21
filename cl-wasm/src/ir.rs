#[derive(Clone, Copy, Debug)]
pub struct InsMovr {
    pub dst: usize,
    pub src: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct InsMovi {
    pub dst: usize,
    pub val: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct InsAdd {
    pub dst: usize,
    pub src_l: usize,
    pub src_r: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct InsSub {
    pub dst: usize,
    pub src_l: usize,
    pub src_r: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct InsMul {
    pub dst: usize,
    pub src_l: usize,
    pub src_r: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct InsDiv {
    pub dst: usize,
    pub src_l: usize,
    pub src_r: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct InsMod {
    pub dst: usize,
    pub src_l: usize,
    pub src_r: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Ins {
    Movr(InsMovr),
    Movi(InsMovi),
    Add(InsAdd),
    Sub(InsSub),
    Mul(InsMul),
    Div(InsDiv),
    Mod(InsMod),
}
