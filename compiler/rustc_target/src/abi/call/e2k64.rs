use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() {
        ret.cast_to(Uniform { unit: Reg::i64(), total: ret.layout.size });
    } else {
        ret.extend_integer_width_to(64);
    }
}

fn classify_arg<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.layout.is_aggregate() {
        arg.cast_to(Uniform { unit: Reg::i64(), total: arg.layout.size });
    } else {
        arg.extend_integer_width_to(64);
    }
}

pub fn compute_abi_info<Ty>(fn_abi: &mut FnAbi<'_, Ty>) {
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(arg);
    }
}
