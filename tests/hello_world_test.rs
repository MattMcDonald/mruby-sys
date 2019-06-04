extern crate mruby_sys;

use mruby_sys::{mrb_close, mrb_load_string, mrb_open};

#[test]
fn hello_world() {
    let s = std::ffi::CString::new("p 'hello world!'").expect("Failed to CString::new");
    unsafe {
        let mrb_state = mrb_open();
        mrb_load_string(mrb_state, s.as_ptr());
        mrb_close(mrb_state);
    }
}
