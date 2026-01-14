use rustc_abi::{BackendRepr, HasDataLayout, Reg, TyAbiInterface};
use crate::callconv::{ArgAbi, FnAbi, Uniform};

fn is_e2k_aggregate<Ty>(arg: &ArgAbi<'_, Ty>) -> bool {
    match arg.layout.backend_repr {
        BackendRepr::SimdVector { .. } => true,
        _ => arg.layout.is_aggregate(),
    }
}

fn classify_ret<'a, Ty, C>(cx: &C, ret: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !ret.layout.is_sized() {
        // Not touching this...
        return;
    }

    if is_e2k_aggregate(ret) || ret.layout.size.bits() > 64 {
        ret.cast_to(Uniform::new(
            Reg::i64(), ret.layout.size.align_to(Reg::i64().align(cx))
        ));
    } else {
        ret.extend_integer_width_to(64);
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !arg.layout.is_sized() {
        // Not touching this...
        return;
    }

    if is_e2k_aggregate(arg) || arg.layout.size.bits() > 64 {
        arg.cast_to(Uniform::new(
            Reg::i64(), arg.layout.size.align_to(Reg::i64().align(cx))
        ));
    } else {
        arg.extend_integer_width_to(64);
    }
}

pub fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret(cx, &mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg);
    }
}
