#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)] // Unused constant errors from bindgen output
mod math;

fn main() {
    let mut res: u32 = 0;
    let mut req = math::math_request_t {
        arg1: 230,
        arg2: 45,
        cmd: math::math_command_t_math_command_add,
        res: &mut res,
    };

    let err_res = unsafe { math::math_execute(req) };

    println!("230+45 = {} | ERROR={}", unsafe { *req.res }, err_res);

    req.cmd = math::math_command_t_math_command_sub;
    let err_res = unsafe { math::math_execute(req) };
    println!("230-45 = {} | ERROR={}", unsafe { *req.res }, err_res);

    req.cmd = 42;
    let err_res = unsafe { math::math_execute(req) };
    println!("unknown cmd = {} | ERROR={}", unsafe { *req.res }, err_res);

    req.cmd = math::math_command_t_math_command_sub;
    req.arg1 = 1;
    req.arg2 = 2;
    let err_res = unsafe { math::math_execute(req) };
    println!("(1-2) = {} | ERROR={}", unsafe { *req.res }, err_res);

    req.cmd = math::math_command_t_math_command_add;
    req.arg1 = u32::max_value();
    req.arg2 = 1;
    let err_res = unsafe { math::math_execute(req) };
    println!("(MAXU32+1) = {} | ERROR={}", unsafe { *req.res }, err_res);
}
