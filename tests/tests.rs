#![feature(proc_macro_hygiene)]
extern crate oxi_gen;
use oxi_gen::oxi;

#[test]
/// The default parser name is `oxi_parser`.
/// If this test compiles, it is successful.
fn it_works() {
  enum Token {}
  oxi! {
    %token Token
    %start Token
  }

  let parser = oxi_parser;
}

#[test]
/// Renaming the parser works.
/// If this test compiles, it is successful.
fn can_give_name_option() {
  enum Token {}
  oxi! {
    %name parser
    %token Token
    %start Token
  }

  let name = parser;
}

#[test]
///
fn integration_test() {
  enum Token {}
  oxi! {
    %name parser
    %public
    %token Token
    %start Token
  }
}
