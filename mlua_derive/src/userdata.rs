use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Index, Lit, Meta, Path,
};

fn is_path_simple_name(path: &Path, name: &str) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path
            .segments
            .first()
            .is_some_and(|seg| seg.arguments.is_none() && seg.ident.to_string().as_str() == name)
}

fn find_fields_with_attr<'a>(
    data: &'a DataStruct,
    attr_name: &'a str,
) -> impl Iterator<Item = (&'a Field, &'a Attribute, usize)> + 'a {
    data.fields.iter().enumerate().filter_map(move |(i, field)| {
        field
            .attrs
            .iter()
            .find(|attr| is_path_simple_name(attr.meta.path(), attr_name))
            .map(|attr| (field, attr, i))
    })
}

fn get_field_name(field: &Field, attr: &Attribute) -> String {
    match &attr.meta {
        Meta::Path(_) => field
            .ident
            .as_ref()
            .expect("#[field] or #[field_mut] without name can only be used on a named field")
            .to_string(),
        Meta::NameValue(_) => panic!(
            "Invalid #[field] or #[field_mut] attribute. Valid variants: #[field], #[field(name = \"name\")]"
        ),
        Meta::List(list) => {
            let nv = list
                    .parse_args::<syn::MetaNameValue>()
                    .expect("Invalid #[field] or #[field_mut] attribute. Valid variants: #[field], #[field(name = \"name\")]");
            if is_path_simple_name(&nv.path, "name") {
                match &nv.value {
                    Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => s.value(),
                    _ => panic!("Invalid field name, must be a string literal!"),
                }
            } else {
                field
                    .ident
                    .as_ref()
                    .expect("#[field] or #[field_mut] without name can only be used on a named field")
                    .to_string()
            }
        }
    }
}

pub fn userdata(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let Data::Struct(data) = data else {
        unimplemented!("derive(UserData) can only be used on structs")
    };

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let where_clause = match &generics.where_clause {
        Some(where_clause) => quote! { #where_clause, Self: ::mlua::FromLua },
        None => quote! { where Self: ::mlua::FromLua },
    };

    let fields = find_fields_with_attr(&data, "field").chain(find_fields_with_attr(&data, "field_mut"));
    let fields_mut = find_fields_with_attr(&data, "field_mut");

    let getters = fields.map(|(field, attr, index)| {
        let name = get_field_name(field, attr);
        let field_span = field.span();

        match &field.ident {
            None => {
                let ident = Index::from(index);
                quote_spanned! {field_span=>
                    fields.add_field_method_get(#name, |_, this| Ok(this.#ident));
                }
            }

            Some(ident) => {
                quote_spanned! {field_span=>
                    fields.add_field_method_get(#name, |_, this| Ok(this.#ident));
                }
            }
        }
    });
    let setters = fields_mut.map(|(field, attr, index)| {
        let name = get_field_name(field, attr);
        let field_span = field.span();

        match &field.ident {
            None => {
                let ident = Index::from(index);
                quote_spanned! {field_span=>
                    fields.add_field_method_set(#name, |_, this, val| {
                        this.#ident = val;
                        Ok(())
                    });
                }
            }

            Some(ident) => {
                quote_spanned! {field_span=>
                    fields.add_field_method_set(#name, |_, this, val| {
                        this.#ident = val;
                        Ok(())
                    });
                }
            }
        }
    });

    quote! {
      impl #impl_generics ::mlua::UserData for #ident #ty_generics #where_clause {
        fn add_fields<F: ::mlua::UserDataFields<Self>>(fields: &mut F) {
            #(#getters)*
            #(#setters)*
        }
        fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {}
      }
    }
    .into()
}
