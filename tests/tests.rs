#![feature(proc_macro_hygiene)]
extern crate oxi_gen;
use oxi_gen::oxi;

#[test]
/// The default parser name is `oxi_parser`.
/// If this test compiles, it is successful.
fn it_works() {
  enum Token {}
  oxi! {
    %token_type Token
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
    %token_type Token
    %start Token
  }

  let name = parser;
}

#[test]
///
fn simple_featured_grammar() {
  enum Token {
    Number(i8),
    Variable(String),
    Plus,
    Times,
    OpenParen,
    CloseParen,
  }

  enum Exp {
    Add(Box<Exp>, Box<Exp>),
    Mult(Box<Exp>, Box<Exp>),
    Int(i8),
    Var(String),
  }

  type BoxedExp = Box<Exp>;

  oxi! {
    %name parser
    %public
    %token_type Token
    %start Exp

    %left times
    %left plus

    Exp: Box<Exp>
      = Term { $1 }
      | Exp Plus Term { Box::new(Exp::Add($1, $3)) }
      ;

    Term: Box<Exp>
      = Factor { $1 }
      | Term Times Factor { Box::new(Exp::Mult($1, $3)) }
      ;

    Factor: Box<Exp>
      = int { Exp::Int($1) }
      | var { Exp::Var($1) }
      | OpenParen Exp CloseParen { $2 }
      ;
  }

  let parser_fn = parser;
}

#[test]
///
fn full_featured_grammar() {
  enum Token {
    Number(i8),
    Variable(String),
    Plus,
    Times,
    OpenParen,
    CloseParen,
  }

  enum Exp {
    Add(Box<Exp>, Box<Exp>),
    Mult(Box<Exp>, Box<Exp>),
    Int(i8),
    Var(String),
  }

  /*oxi! {
    %name parser
    %public
    %token_type Token
    %start Exp

    %token {
      int Token::Number($$)
      var Token::Variable($$)
      '+' Token::Plus
      '*' Token::Times
      '(' Token::OpenParen
      ')' Token::CloseParen
    }

    %left times
    %left plus

    Exp: Exp
      = Term { $1 }
      | Exp '+' Term { Exp::Add($1, $3) }
      ;

    Term: Exp
      = Factor { $1 }
      | Term '*' Factor { Exp::Mult($1, $3) }
      ;

    Factor: Exp
      = int { Exp::Int($1) }
      | var { Exp::Var($1) }
      | '(' Exp ')' { $2 }
      ;
  }*/
}
