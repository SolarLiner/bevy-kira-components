use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn;
use syn::visit::Visit;
use syn::{visit, Field, Ident, Index, Member, PathSegment};

#[proc_macro_derive(EffectRack)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_create(&ast)
}

#[derive(Default)]
struct DeriveVisitor {
    members: Vec<Member>,
    handle_type_idents: Vec<Ident>,
}

impl<'ast> Visit<'ast> for DeriveVisitor {
    fn visit_field(&mut self, node: &'ast Field) {
        if let Some(ident) = node.ident.clone() {
            self.members.push(ident.into());
        } else {
            self.members
                .push(Member::Unnamed(Index::from(self.members.len())));
        }

        visit::visit_field(self, node);
    }

    fn visit_path_segment(&mut self, node: &'ast PathSegment) {
        let ident_string = node.ident.to_string();
        let stripped = ident_string
            .strip_suffix("Builder")
            .expect("All types in an EffectRack must be effect builders ending with 'Builder'.");

        let type_ident = format_ident!("{}Handle", stripped);
        self.handle_type_idents.push(type_ident);
        visit::visit_path_segment(self, node);
    }
}

fn impl_create(ast: &syn::DeriveInput) -> TokenStream {
    // Name of the effect rack type
    let rack_ident = &ast.ident;

    // Collect all the Fields
    let mut visitor = DeriveVisitor::default();
    visitor.visit_derive_input(&ast);

    let controller_ident = Ident::new(&format!("{}Controller", rack_ident), Span::call_site());
    let field_name_idents = visitor.members;
    let handle_type_idents = visitor.handle_type_idents;

    // Generate the controller type depending on if the effect rack type was a normal struct
    // or a tuple struct.
    let struct_gen = match field_name_idents[0] {
        Member::Named(_) => {
            quote! {
                #[derive(Component)]
                struct #controller_ident {
                    #(#field_name_idents : #handle_type_idents),*
                }
            }
        }
        Member::Unnamed(_) => {
            quote! {
                #[derive(Component)]
                struct #controller_ident (
                    #(#handle_type_idents),*
                );
            }
        }
    };

    let impl_gen = quote! {
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

    let tokens = quote! {
        #struct_gen
        #impl_gen
    };

    tokens.into()
}
