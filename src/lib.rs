use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, ImplGenerics, Type, TypeGenerics,
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
/// #[derive(FluentFieldAssertions)]
/// struct User {
///     id: usize,
///     name: String,
///     age: usize,
///     // You can skip assertions for some fields.
///     #[assertions(skip)]
///     score: f64,
/// }
///
/// let user = User {
///     id: 1,
///     name: "Alice".to_string(),
///     age: 17,
///     score: 95.0,
/// };
///
/// user.id_eq(&1)
///     .name_eq(&"Alice".to_string())
///     .name_ne(&"Bob".to_string())
///     .name_satisfies(|name| name.starts_with("A"))
///     .age_eq(&17);
/// ```
///
/// # Example for generics struct
/// ```rust
/// use core::fmt::Debug;
/// use fluent_field_assertions::FluentFieldAssertions;
///
/// #[derive(FluentFieldAssertions)]
/// struct Point<T>
/// // Generics type `T` must implement trait `Eq` and `Debug`.
/// where
///     T: Eq + Debug,
/// {
///     x: T,
///     y: T,
///     z: T,
///     // You can skip assertions for some fields.
///     #[assertions(skip)]
///     t: T,
/// }
///
/// let point = Point { x: 1, y: 2, z: 3, t: 4 };
///
/// point.x_eq(&1)
///     .y_ne(&9) // Note that `y` is NOT `9`.
///     .z_satisfies(|z| z > &0);
/// ```
#[proc_macro_derive(FluentFieldAssertions, attributes(assertions))]
pub fn fluent_field_assertions(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let type_name = &ast.ident;

    let gen = if let Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        let method_tokens = fields.iter().flat_map(generate_methods).collect();

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

fn generate_methods(field: &Field) -> Vec<TokenStream2> {
    let field_name = field
        .clone()
        .ident
        .unwrap_or_else(|| panic!("Field name must be present."));
    let field_type = field.ty.clone();

    let skip = field.attrs.iter().any(|attr| {
        attr.path().is_ident("assertions")
            && attr
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        Ok(())
                    } else {
                        Err(meta.error("unrecognized repr"))
                    }
                })
                .is_ok()
    });

    if skip {
        vec![]
    } else {
        vec![
            generate_eq_method(&field_name, &field_type),
            generate_ne_method(&field_name, &field_type),
            generate_satisfies_method(&field_name, &field_type),
        ]
    }
}

fn generate_eq_method(field_name: &Ident, field_type: &Type) -> TokenStream2 {
    let method_name = Ident::new(&format!("{}_eq", field_name), Span::call_site());

    quote! {
        #[inline(always)]
        #[track_caller]
        pub fn #method_name(&self, expected: &#field_type) -> &Self
        {
            assert_eq!(&self.#field_name, expected);
            self
        }
    }
}

fn generate_ne_method(field_name: &Ident, field_type: &Type) -> TokenStream2 {
    let method_name = Ident::new(&format!("{}_ne", field_name), Span::call_site());

    quote! {
        #[inline(always)]
        #[track_caller]
        pub fn #method_name(&self, expected: &#field_type) -> &Self
        {
            assert_ne!(&self.#field_name, expected);
            self
        }
    }
}

fn generate_satisfies_method(field_name: &Ident, field_type: &Type) -> TokenStream2 {
    let method_name = Ident::new(&format!("{}_satisfies", field_name), Span::call_site());

    quote! {
        #[inline(always)]
        #[track_caller]
        pub fn #method_name(&self, pred: impl FnOnce(&#field_type) -> bool) -> &Self {
            assert!(pred(&self.#field_name));
            self
        }
    }
}
