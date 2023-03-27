#[derive(PartialOrd, PartialEq, Clone, Copy)]
enum ReturnCode {
    Success = 0,
    Warning = 1,
    Critical = 2,
}

impl ReturnCode {
    fn set_if_higher<'a>(mut self: &'a mut ReturnCode, exit_code: &'a mut ReturnCode) {
        if *exit_code as i32 > *self as i32 {
            println!("Changing RC: {:?} -> {:?}", *exit_code as i32, *self as i32);
            self = exit_code;
        }
    }
}

fn main() {
    for rc in [ReturnCode::Success, ReturnCode::Warning, ReturnCode::Critical] {
        println!("{:?}", rc as i32);
    }

    let mut return_code = ReturnCode::Success;
    return_code.set_if_higher(&mut ReturnCode::Warning);
    println!("{:?}", return_code as i32);
}