#[derive(PartialOrd, PartialEq, Clone, Copy)]
enum ReturnCode {
    Success = 0,
    Warning = 1,
    Critical = 2,
}

impl ReturnCode {
    fn set_if_higher(&mut self, mut exit_code: ReturnCode) {
        if exit_code as i32 > *self as i32 {
            self = &mut exit_code;
        }
    }
}

fn main() {
    for rc in ReturnCode::Success as i32..ReturnCode::Critical as i32 {
        println!("{:?}", rc);
    }

    let mut return_code = ReturnCode::Success;
    return_code.set_if_higher(ReturnCode::Warning);
    println!("{:?}", return_code as i32);
}