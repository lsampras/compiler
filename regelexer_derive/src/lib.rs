#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate proc_macro;

use proc_macro::{TokenStream, Group, Delimiter, TokenTree, Literal};
use quote::quote;
use syn;
use syn::{Data, DeriveInput, DataEnum, Ident};
use syn::token::Enum;

#[proc_macro_derive(LexerMacro, attributes(regex,token,skip))]
pub fn lexer_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let data: DataEnum = match ast.data {
        Data::Enum(d) => d,
        _ => panic!("not an enum")
    };

    let mut regexes:Vec<String> = vec![];
    let mut literals:Vec<String>  = vec![];
    let mut mappings:Vec<(String,Ident)>  = vec![];
    let mut skips:Vec<Ident>  = vec![];

    for i in data.variants {
        let tokentype = &i.attrs[0].path.segments[0].ident;
        let tokens: TokenStream = format!("{}",i.attrs[0].tokens).parse().unwrap();
        let tree : Vec<TokenTree> = tokens.into_iter().collect();
        let group = match &tree.get(0) {
            Some(TokenTree::Group(lit)) => lit,
            _ => panic!("p1")
        };
        let group_tokens: Vec<TokenTree> = group.stream().into_iter().collect();
        let mut literal: String = match group_tokens.get(0) {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            _ => panic!("p2")
        };
        let literal2: String = match group_tokens.get(2) {
            Some(TokenTree::Literal(lit)) => lit.to_string(),
            Some(TokenTree::Ident(lit)) => lit.to_string(),
            _ => "".to_string()
        };
        literal = literal.get(1..(literal.len()-1)).unwrap().to_string();
        mappings.push((literal.clone(), i.ident.clone()));
        if tokentype == "token" {
            println!("{}","token error");
            literals.push(literal);
        }
        else if tokentype == "regex" {
            println!("{}","regex error");
            regexes.push(literal);
        }
        if literal2 == "skip" {
            skips.push(i.ident);
        }
    }
    let name = &ast.ident;
    let mut skipstream = quote! {
    };
    for token in skips {
        skipstream = quote! {
            &#name::#token,
        };
    }
    let mut regexstream = quote! {
    };
    for token in regexes {
        regexstream = quote! {
            #token.to_string(),
        };
    }
    let mut literalstream = quote! {
    };
    for token in literals {
        literalstream = quote! {
            #token.to_string(),
        };
    }
    let mut mappingstream = quote! {
    };
    for (token,text) in mappings {

        mappingstream = quote! {
            #token => &#name::#text,
            #mappingstream
        };
    }

    let gen = quote! {
        impl LexerEnumTrait for #name {
            fn get_enum_from_pattern(input: &str) -> &'static Self {
                match input {
                    #mappingstream
                    _ => panic!("not found")
                }
            }
            fn get_skips() -> Vec<&'static Self> {
                vec![
                    #skipstream
                ]
            }
            fn get_regexes() -> Vec<String> {
                vec![
                    #regexstream
                ]
            }
            fn get_literals() -> Vec<String> {
                vec![
                    #literalstream
                ]
            }
        }
    };
    gen.into()
}