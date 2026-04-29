use crate::ir::IRInst;
use crate::ast::BinOp;

pub fn constant_fold(ir: Vec<IRInst>) -> Vec<IRInst> {
    let mut optimized = Vec::new();
    for inst in ir {
        match inst {
            IRInst::BinOp(dest, op, left, right) => {
                // Try to fold if both operands are constants
                if let (Some(lval), Some(rval)) = (get_constant(&left, &optimized), get_constant(&right, &optimized)) {
                    let result = match op {
                        BinOp::Add => lval + rval,
                        BinOp::Sub => lval - rval,
                        BinOp::Mul => lval * rval,
                        BinOp::Div => if rval != 0 { lval / rval } else { 0 },
                    };
                    optimized.push(IRInst::LoadConst(dest, result));
                } else {
                    optimized.push(IRInst::BinOp(dest, op, left, right));
                }
            }
            _ => optimized.push(inst),
        }
    }
    optimized
}

// Helper: check if a variable is a constant by scanning previous instructions
fn get_constant(var: &str, code: &[IRInst]) -> Option<i32> {
    for inst in code.iter().rev() {
        match inst {
            IRInst::LoadConst(dest, val) if dest == var => return Some(*val),
            _ => continue,
        }
    }
    None
}