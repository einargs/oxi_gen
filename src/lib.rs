#![feature(extern_crate_item_prelude)]

// This is the version of proc_macro included with rust itself
extern crate proc_macro;

// This is the library that both quote and syn depend on
extern crate proc_macro2;
extern crate syn;
extern crate quote;

use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
use syn::{Token, parse_macro_input, Visibility};
use quote::{ToTokens, quote};


enum ConfigOption {
  /// The name of the generated parser function
  /// Defaults to `oxi_parser`
  Name(Ident),
  /// The visibility of the generated parser function
  Visibility(Visibility),
  /// The type used to represent the tokens
  Token(Ident),
  /// The entry symbol the parser should start at.
  /// In other words, this is the root AST node and
  /// the type the parser will return.
  Start(Ident),
}

impl Parse for ConfigOption {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![%]>()?;
    let config_option: Ident = input.parse()?;

    match config_option.to_string().as_ref() {
      "name" => {
        Ok(ConfigOption::Name(input.parse()?))
      },
      "visibility" => {
        Ok(ConfigOption::Visibility(input.parse()?))
      },
      "token" => {
        Ok(ConfigOption::Token(input.parse()?))
      },
      "start" => {
        Ok(ConfigOption::Start(input.parse()?))
      },
      _ => {
        panic!("FIX THIS LATER PART 1: MESSAGES I WILL REGRET");
      },
    }
  }
}

enum Term {
  ConfigOption(ConfigOption),
  Production,
}

impl Term {
  fn is_config_option(&self) -> bool {
    match self {
      Term::ConfigOption(_) => true,
      _ => false,
    }
  }

  fn is_production(&self) -> bool {
    match self {
      Term::Production => true,
      _ => false,
    }
  }
}

impl Parse for Term {
  fn parse(input: ParseStream) -> Result<Self> {
    let term = if input.peek(Token![%]) {
      Term::ConfigOption(input.parse()?)
    } else {
      Term::Production
    };

    Ok(term)
  }
}

fn parse_terms(input: ParseStream) -> Vec<Term> {
  let mut terms = vec![];


  for i in 0..100 {
    match input.parse::<Term>() {
      Ok(term) => terms.push(term),
      Err(_) => return terms,
    };
  }
  println!("CURSED");

  return terms;
}

struct OxiConfig {
  name: Ident,
}

struct OxiSyntax {
  config: OxiConfig,
}

impl Parse for OxiSyntax {
  fn parse(input: ParseStream) -> Result<Self> {
    let terms = parse_terms(input);
    let config_options =
      terms.into_iter()
      .filter_map(|term| match term {
        Term::ConfigOption(cfg) => Some(cfg),
        Term::Production => None,
      });

    let parser_name: Ident =
      config_options.into_iter().find_map(|cfg| {
        if let ConfigOption::Name(ident) = cfg {
          Some(ident)
        } else {
          None
        }
      })
      .unwrap_or_else(||
        Ident::new("oxi_parser", Span::call_site())
      );

    Ok(OxiSyntax {
      config: OxiConfig {
        name: parser_name,
      },
    })
  }
}

#[proc_macro]
pub fn oxi(input: TokenStream) -> TokenStream {
  let OxiSyntax {
    config: OxiConfig {
      name,
    },
  } = parse_macro_input!(input as OxiSyntax);

  let tokens = quote! {
    fn #name() -> bool {
      true
    }
  };

  tokens.into()
}
