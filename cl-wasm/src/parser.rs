use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ir;

pub fn parse_file(path: &str) -> Vec<ir::Ins> {
    let is = BufReader::new(File::open(path).expect("Failed to open source file"));
    let mut res = vec![];

    for line in is.lines() {
        let line = line.unwrap();
        let line: Vec<_> = line.split_whitespace().collect();
        if line.len() == 0 {
            continue;
        }

        let cmd = line.first().unwrap();
        let ins = match *cmd {
            "movr" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                ir::Ins::Movr(ir::InsMovr { dst, src })
            }

            "movi" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let val: i32 = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                ir::Ins::Movi(ir::InsMovi { dst, val })
            }

            "add" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src_l: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                let src_r: usize = line
                    .get(3)
                    .expect("Missing third argument")
                    .parse()
                    .expect("Third argument must be an int");
                ir::Ins::Add(ir::InsAdd { dst, src_l, src_r })
            }

            "sub" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src_l: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                let src_r: usize = line
                    .get(3)
                    .expect("Missing third argument")
                    .parse()
                    .expect("Third argument must be an int");
                ir::Ins::Sub(ir::InsSub { dst, src_l, src_r })
            }

            "mul" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src_l: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                let src_r: usize = line
                    .get(3)
                    .expect("Missing third argument")
                    .parse()
                    .expect("Third argument must be an int");
                ir::Ins::Mul(ir::InsMul { dst, src_l, src_r })
            }

            "div" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src_l: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                let src_r: usize = line
                    .get(3)
                    .expect("Missing third argument")
                    .parse()
                    .expect("Third argument must be an int");
                ir::Ins::Div(ir::InsDiv { dst, src_l, src_r })
            }

            "mod" => {
                let dst: usize = line
                    .get(1)
                    .expect("Missing first argument")
                    .parse()
                    .expect("First argument must be an int");
                let src_l: usize = line
                    .get(2)
                    .expect("Missing second argument")
                    .parse()
                    .expect("Second argument must be an int");
                let src_r: usize = line
                    .get(3)
                    .expect("Missing third argument")
                    .parse()
                    .expect("Third argument must be an int");
                ir::Ins::Mod(ir::InsMod { dst, src_l, src_r })
            }

            _ => panic!("Unknown instruction '{}'", cmd),
        };

        res.push(ins);
    }

    res
}
