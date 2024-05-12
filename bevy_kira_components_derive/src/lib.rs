use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn;
use syn::{Data, Fields, Ident, Type};

#[proc_macro_derive(EffectRack)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_create(&ast)
}

fn impl_create(ast: &syn::DeriveInput) -> TokenStream {
    let rack_ident = &ast.ident;
    let data = &ast.data;

    let mut fields = &Fields::Unit;

    match data {
        Data::Struct(data_struct) => fields = &data_struct.fields,
        _ => {}
    }

    let mut builder_type_idents: Vec<&Ident> = Vec::new();
    let mut field_name_idents: Vec<&Ident> = Vec::new();

    // todo: Take tuple structs into account
    // todo: Panic messages
    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                if let Some(ident) = &field.ident {
                    field_name_idents.push(ident);
                } else {
                    panic!();
                }

                match &field.ty {
                    Type::Path(type_path) => {
                        for segment in &type_path.path.segments {
                            builder_type_idents.push(&segment.ident);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    let handle_type_idents: Vec<Ident> = builder_type_idents
        .iter()
        .map(|ident| {
            let string = ident.to_string();
            let stripped = string.strip_suffix("Builder").expect(
                "All types in an EffectRack must be effect builders ending with 'Builder'.",
            );

            format_ident!("{}Handle", stripped)
        })
        .collect();

    let controller_ident = Ident::new(&format!("{}Controller", rack_ident), Span::call_site());

    let gen = quote! {
        #[derive(Component)]
        struct #controller_ident {
            #(#field_name_idents : #handle_type_idents),*
        }

        impl EffectRack for #rack_ident {
            type Controller = #controller_ident;

            fn apply(self, mut track_builder: TrackBuilder) -> (Self::Controller, TrackBuilderWrapped) {
                (
                    Self::Controller {
                        #(#field_name_idents: track_builder.add_effect(self.#field_name_idents)),*
                    },
                    TrackBuilderWrapped(track_builder),
                )
            }
        }
    };

    gen.into()
}
