use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(PartialOrd)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let item: syn::Item = input.into();
    let syn::Item::Struct(item_struct) = item else {
        panic!("Strict PartialOrd only support struct inputs");
    };

    derive_partial_ord(&item_struct).into()
}


fn derive_partial_ord(input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let members = input.fields.members();
    let order_varname = format_ident!("{}", "order");
    let accumulator = quote! { let mut #order_varname = std::cmp::Ordering::Equal; };
    let members_match_blocks = members.map(|member| {
        quote! {
            let member_ord = self.#member.partial_cmp(&other.#member)?;
            match (#order_varname, member_ord) {
                (std::cmp::Ordering::Equal, x) => #order_varname = x,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => (),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => (),
                _ => return None,
            };
        }
    });
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics std::cmp::PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                #accumulator
                #(#members_match_blocks)*

                Some(#order_varname)
            }
        }
    }
}
