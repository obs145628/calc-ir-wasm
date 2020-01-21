use binaryen::*;

use crate::ir;

struct CodeGen {
    exprs: Vec<Expr>,
    max_local: Option<usize>,
    module: Module,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            exprs: vec![],
            max_local: None,
            module: Module::new(),
        }
    }

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {
        let val = self.module.get_local(0, ValueTy::I32);
        self.exprs.push(val);

        let mut exprs = vec![];
        std::mem::swap(&mut self.exprs, &mut exprs);
        let exprs = self.module.block(None as Option<&str>, exprs, None);

        let mut locals = vec![];
        match self.max_local {
            Some(max_local) => {
                for _ in 0..max_local + 1 {
                    locals.push(ValueTy::I32);
                }
            }
            _ => {}
        }

        let fn_type = self.module.add_fn_type(Some("i"), &[], Ty::I32);
        self.module.add_fn("run", &fn_type, &locals, exprs);
        self.module.add_fn_export("run", "run");
        if !self.module.is_valid() {
            panic!("Invalid module");
        }
    }

    pub fn compile(&mut self, code: &Vec<ir::Ins>) {
        for ins in code {
            self.compile_ins(ins);
        }
    }

    pub fn get_module(self) -> Module {
        self.module
    }

    fn use_local(&mut self, id: usize) {
        if self.max_local.is_none() {
            self.max_local = Some(0);
        }
        if id > self.max_local.unwrap() {
            self.max_local = Some(id)
        }
    }

    fn compile_ins(&mut self, ins: &ir::Ins) {
        match ins {
            ir::Ins::Movr(ins) => self.compile_movr(ins),
            ir::Ins::Movi(ins) => self.compile_movi(ins),
            ir::Ins::Add(ins) => self.compile_add(ins),
            ir::Ins::Sub(ins) => self.compile_sub(ins),
            ir::Ins::Mul(ins) => self.compile_mul(ins),
            ir::Ins::Div(ins) => self.compile_div(ins),
            ir::Ins::Mod(ins) => self.compile_mod(ins),
        }
    }

    fn compile_movr(&mut self, ins: &ir::InsMovr) {
        let val = self.module.get_local(ins.src as u32, ValueTy::I32);
        let setl = self.module.set_local(ins.dst as u32, val);
        self.exprs.push(setl);
        self.use_local(ins.src);
        self.use_local(ins.dst);
    }

    fn compile_movi(&mut self, ins: &ir::InsMovi) {
        let val = self.module.const_(Literal::I32(ins.val as u32));
        let setl = self.module.set_local(ins.dst as u32, val);
        self.exprs.push(setl);
        self.use_local(ins.dst);
    }

    fn compile_binop(&mut self, op: BinaryOp, src_l: usize, src_r: usize, dst: usize) {
        let val_l = self.module.get_local(src_l as u32, ValueTy::I32);
        let val_r = self.module.get_local(src_r as u32, ValueTy::I32);
        let val = self.module.binary(op, val_l, val_r);
        let setl = self.module.set_local(dst as u32, val);
        self.exprs.push(setl);
        self.use_local(src_l);
        self.use_local(src_r);
        self.use_local(dst);
    }

    fn compile_add(&mut self, ins: &ir::InsAdd) {
        self.compile_binop(BinaryOp::AddI32, ins.src_l, ins.src_r, ins.dst);
    }

    fn compile_sub(&mut self, ins: &ir::InsSub) {
        self.compile_binop(BinaryOp::SubI32, ins.src_l, ins.src_r, ins.dst);
    }

    fn compile_mul(&mut self, ins: &ir::InsMul) {
        self.compile_binop(BinaryOp::MulI32, ins.src_l, ins.src_r, ins.dst);
    }

    fn compile_div(&mut self, ins: &ir::InsDiv) {
        self.compile_binop(BinaryOp::DivSI32, ins.src_l, ins.src_r, ins.dst);
    }

    fn compile_mod(&mut self, ins: &ir::InsMod) {
        self.compile_binop(BinaryOp::RemSI32, ins.src_l, ins.src_r, ins.dst);
    }
}

pub fn build_module(code: &Vec<ir::Ins>) -> Module {
    let mut cl = CodeGen::new();
    cl.begin();
    cl.compile(code);
    cl.end();
    cl.get_module()
}
