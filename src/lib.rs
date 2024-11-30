use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, Data, DeriveInput, Fields, LitStr, TypePath,
};

#[proc_macro_derive(StructIter, attributes(iter))]
pub fn iterable_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut trait_path = syn::parse_str::<TypePath>("std::fmt::Debug").unwrap();
    let res = input.attrs.iter().try_for_each(|attr| {
        if attr.path().is_ident("iter") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("trait") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;

                    trait_path = s.parse::<TypePath>()?;
                    Ok(())
                } else {
                    Err(meta.error("Unsupported attribute"))
                }
            })
        } else {
            Ok(())
        }
    });

    res.unwrap();

    let fields = match &input.data {
        Data::Struct(data_struct) => match data_struct.clone().fields {
            Fields::Named(fields) => &fields.clone().named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().expect("Field must have a name"))
        .collect();

    let expanded = quote! {

        impl<'a> IntoIterator for &'a #name {
            type Item = &'a dyn #trait_path;
            type IntoIter = std::vec::IntoIter<&'a dyn #trait_path>;


            fn into_iter(self) -> Self::IntoIter {
                // Explicitly define the type of the vector
                vec![
                    #(
                        &self.#field_names as &dyn #trait_path
                ),*
                ].into_iter()
            }
        }




        impl IntoIterator for #name
        {
            type Item = std::boxed::Box<dyn #trait_path>;
            type IntoIter = std::vec::IntoIter<std::boxed::Box<dyn #trait_path>>;

            fn into_iter(self) -> Self::IntoIter {
                let vec = vec![
                    #(
                        std::boxed::Box::new(self.#field_names) as std::boxed::Box<dyn #trait_path>
                    ),*                 ];
                vec.into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}
