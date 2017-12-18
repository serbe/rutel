#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::BTreeMap;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(GetSet)]
pub fn qqq(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let attrs = get_attributes(&ast.attrs);
    let method: String = match attrs.get("method") {
        Ok(v) => v,
        _ => String::new(),
    };
    let struct_name = &ast.ident;
    let struct_name2 = struct_name.clone();
    if let syn::Body::Struct(s) = ast.body {
        let field_names: Vec<_> = s.fields().iter().map(|ref x| x.ident.clone().unwrap()).collect();
        let field_names_opt: Vec<_> = s.fields().iter().filter(|ref x| {
            match x.ty.clone() {
                syn::Ty::Path(_, path) => {
                    "Option" == format!("{}", path.segments[0].ident)
                }
                _ => false,
            }
        }).map(|ref x| x.ident.clone().unwrap()).collect();
        let field_names_no_opt: Vec<_> = s.fields().iter().filter(|ref x| {
            match x.ty.clone() {
                syn::Ty::Path(_, path) => {
                    "Option" != format!("{}", path.segments[0].ident)
                }
                _ => false,
            }
        }).map(|ref x| x.ident.clone().unwrap()).collect();
        let field_getter_names = field_names.iter().map(|ref x|
            syn::Ident::new(format!("get_{}", x).as_str()));
        let field_setter_names = field_names.iter().map(|ref x|
            syn::Ident::new(format!("{}", x).as_str()));
        let field_types: Vec<_> = s.fields().iter().map(|ref x| x.ty.clone()).collect();
        let field_types_no_opt: Vec<_> = s.fields().iter().filter(|ref x| {
            match x.ty.clone() {
                syn::Ty::Path(_, path) => {
                    "Option" != format!("{}", path.segments[0].ident)
                }
                _ => false,
            }
        }).map(|ref x| x.ty.clone()).collect();
        let field_names_no_opt2 = field_names_no_opt.clone();
        let field_names_opt3 = field_names_opt.clone();
        let field_names2 = field_names.clone();
        let field_names3 = field_names.clone();
        let field_types2 = field_types.clone();
        let quoted_code = quote! {
            #[allow(dead_code)]
            impl #struct_name {
                pub fn new(#(#field_names_no_opt: #field_types_no_opt,)*) -> Self {
                    #struct_name2{
                        #(#field_names_no_opt2,)*
                        #(#field_names_opt3: None,)*
                    }
                }

                #(
                    pub fn #field_getter_names(&self) -> &#field_types {
                        &self.#field_names2
                    }

                    pub fn #field_setter_names(&mut self, x : #field_types2) -> &mut Self {
                        self.#field_names3 = x;
                        self
                    }
                )*

                pub fn method(&self) -> String {
                    #method.to_string()
                }

                pub fn json(&self) -> String {
                    to_string(self).unwrap()
                }
            }
        };
        println!("{:?}", quoted_code);
        return quoted_code.parse().unwrap();
    }
    "".parse().unwrap()
}

fn get_attributes(attributes: &[syn::Attribute]) -> BTreeMap<String, String> {
    attributes.iter().fold(BTreeMap::new(), |mut btm, attr| {
        if let syn::MetaItem::NameValue(ref name, ref value) = attr.value {
            if let syn::Lit::Str(val, _) = value.clone() {
                println!("{} {}" , name.to_string(), val.clone());
                btm.insert(name.to_string(), val.clone());
            };
        };
        btm
    })
}