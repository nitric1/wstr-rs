#[macro_use]
extern crate proc_macro_hack;
extern crate syn;

use syn::{TokenTree, Token, Lit};

fn input_to_utf16_vec(input: &str) -> Vec<u16> {
    let ast = &syn::parse_token_trees(input).unwrap()[0];
    if let TokenTree::Token(Token::Literal(Lit::Str(ref s, _))) = *ast {
        s.encode_utf16().collect()
    } else {
        panic!("input must be string literal");
    }
}

fn generate_u16_str_arr_code(input: &str) -> String {
    let utf16_vec = input_to_utf16_vec(input);
    format!("{{ static _U16_STR_ARR: [u16; {}] = {:?}; &_U16_STR_ARR }}", utf16_vec.len(), utf16_vec)
}

proc_macro_expr_impl! {
    pub fn wstr_impl(input: &str) -> String {
        generate_u16_str_arr_code(input)
    }
}
