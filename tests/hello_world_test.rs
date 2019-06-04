extern crate mruby_sys;

use mruby_sys::mrb_open;
#[test]
fn hello_world() {
  unsafe{
    let _mrb_state = mrb_open();
  }
}
