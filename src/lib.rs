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
  /// The visibility of the generated parser function.
  Public,
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
      "public" => {
        Ok(ConfigOption::Public)
      },
      "token" => {
        Ok(ConfigOption::Token(input.parse()?))
      },
      "start" => {
        Ok(ConfigOption::Start(input.parse()?))
      },
      _ => {
        // TODO: actually return an error
        panic!("Error: unknown config name");
      },
    }
  }
}

///
struct Production {
  ident: Ident,
}

impl Parse for Production {
  fn parse(input: ParseStream) -> Result<Self> {
    let ident: Ident = input.parse()?;

    Ok(Production {
      ident
    })
  }
}

enum Term {
  ConfigOption(ConfigOption),
  Production(Production),
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
      Term::Production(_) => true,
      _ => false,
    }
  }

  fn config_option(&self) -> Option<&ConfigOption> {
    match self {
      Term::ConfigOption(cfg) => Some(cfg),
      _ => None,
    }
  }

  fn production(&self) -> Option<&Production> {
    match self {
      Term::Production(prod) => Some(prod),
      _ => None,
    }
  }
}

impl Parse for Term {
  fn parse(input: ParseStream) -> Result<Self> {
    let term = if input.peek(Token![%]) {
      Term::ConfigOption(input.parse()?)
    } else {
      Term::Production(input.parse()?)
    };

    Ok(term)
  }
}

fn parse_terms(input: ParseStream) -> Vec<Term> {
  let mut terms = vec![];

  // TODO: this doesn't end when terms stop being read in,
  // hence the for-loop.
  let cursor = input.cursor();
  while !cursor.eof() {
    match input.parse::<Term>() {
      Ok(term) => terms.push(term),
      Err(_) => break,
    };
  }

  return terms;
}

struct OxiConfig {
  /// The name the generated parser function should have.
  name: Ident,
  /// Whether or not the generated parser function should
  /// have the `pub` modifier in front of it.
  public: bool,
  /// The type of the tokens that are passed to the parser.
  token_type: Ident,
  /// The entry production.
  /// Note that this is also the return type.
  start_production: Ident,
}

impl OxiConfig {
  fn from_terms(terms: &[Term]) -> Self {
    let config_options =
      terms.into_iter()
      .filter_map(Term::config_option);

    // Begin with the default values.
    // TODO: if any of the defaults are computation intensive,
    // consider having a second, function-scoped struct type
    // that 
    struct TempOxiConfig {
      name: Option<Ident>,
      public: Option<bool>,
      token_type: Option<Ident>,
      start_production: Option<Ident>,
    }
      
    let mut config = TempOxiConfig {
      name: None,
      public: None,
      start_production: None,
      token_type: None,
    };

    for config_opt in config_options {
      match config_opt {
        ConfigOption::Name(ident) => {
          config.name = Some(ident.clone());
        },
        ConfigOption::Start(ident) => {
          config.start_production = Some(ident.clone());
        },
        ConfigOption::Token(ident) => {
          config.token_type = Some(ident.clone());
        },
        ConfigOption::Public => {
          config.public = Some(true);
        },
      };
    }

    OxiConfig {
      public: config.public.unwrap_or(false),
      name: config.name.unwrap_or_else(|| {
        Ident::new("oxi_parser", Span::call_site())
      }),
      token_type: config.token_type.expect(
        "%token config is required"
      ),
      start_production: config.start_production.unwrap_or_else(|| {
        terms.iter()
          .filter_map(Term::production)
          .map(|Production { ident }| ident)
          .next()
          .expect("Macro should have at least one production")
          .clone()
      }),
    }
  }
}

struct OxiSyntax {
  config: OxiConfig,
}

impl Parse for OxiSyntax {
  fn parse(input: ParseStream) -> Result<Self> {
    let terms = parse_terms(input);

    Ok(OxiSyntax {
      config: OxiConfig::from_terms(&terms),
    })
  }
}

#[proc_macro]
pub fn oxi(input: TokenStream) -> TokenStream {
  let OxiSyntax {
    config: OxiConfig {
      name,
      public,
      start_production, 
      token_type,
    },
  } = parse_macro_input!(input as OxiSyntax);

  let tokens = quote! {
    fn #name() -> #start_production {
      panic!("Have not yet implemented the parser generator");
    }
  };

  tokens.into()
}
