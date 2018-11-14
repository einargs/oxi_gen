#![feature(proc_macro_hygiene)]
extern crate oxi_gen;
use oxi_gen::oxi;

#[test]
/// The default parser name is `oxi_parser`.
/// If this test compiles, it is successful.
fn it_works() {
  oxi! { }

  let parser = oxi_parser;
}

#[test]
/// Renaming the parser works.
/// If this test compiles, it is successful.
fn can_give_name_option() {
  oxi! {
    %name parser
  }

  let name = parser;
}
