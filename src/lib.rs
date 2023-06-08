use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, ImplGenerics, TypeGenerics,
    WhereClause,
};

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

/// FluentFieldAssertions provide fluent assertions for struct.
///
/// # Example for struct
/// ```rust
/// use fluent_field_assertions::FluentFieldAssertions;
///
/// #[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
/// struct User {
///     id: usize,
///     name: String,
/// }
///
/// User {
///     id: 1,
///     name: "Alice".to_string(),
/// }
/// .id_eq(1)
/// .name_eq("Alice".to_string());
/// ```
///
/// # Example for generics struct
/// ```rust
/// use core::fmt::Debug;
/// use fluent_field_assertions::FluentFieldAssertions;
///
/// #[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
/// struct Point<T>
/// where
///     T: Eq + Debug,
/// {
///     x: T,
///     y: T,
/// }
///
/// Point { x: 1, y: 2 }.x_eq(1).y_eq(2);
/// ```
#[proc_macro_derive(FluentFieldAssertions)]
pub fn fluent_field_assertions(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let type_name = &ast.ident;

    let gen = if let Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        let method_tokens = fields.iter().map(|field| generate_method(field)).collect();

        generate_impl(
            impl_generics,
            type_name,
            ty_generics,
            where_clause,
            method_tokens,
        )
    } else {
        panic!("#[derive(FluentFieldAssertions)] is only defined for structs.");
    };

    gen.into()
}

fn generate_impl(
    impl_generics: &ImplGenerics,
    type_name: &Ident,
    ty_generics: &TypeGenerics,
    where_clause: &Option<&WhereClause>,
    method_tokens: Vec<TokenStream2>,
) -> TokenStream2 {
    quote! {
        impl #impl_generics #type_name #ty_generics #where_clause {
            #(#method_tokens)*
        }
    }
}

fn generate_method(field: &Field) -> TokenStream2 {
    let field_name = field
        .clone()
        .ident
        .unwrap_or_else(|| panic!("Field name must be present."));
    let method_name = Ident::new(&format!("{}_eq", field_name), Span::call_site());
    let field_type = field.ty.clone();

    quote! {
        #[inline(always)]
        fn #method_name(&self, expected: #field_type) -> &Self
        where
            #field_type: Eq + core::fmt::Debug
        {
            assert_eq!(self.#field_name, expected);
            self
        }
    }
}
