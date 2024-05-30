use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn;
use syn::visit::Visit;
use syn::{visit, Field, Ident, PathSegment};

#[proc_macro_derive(EffectRack)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_create(&ast)
}

struct DeriveVisitor {
    field_name_idents: Vec<Ident>,
    handle_type_idents: Vec<Ident>,
}

impl<'ast> Visit<'ast> for DeriveVisitor {
    fn visit_field(&mut self, node: &'ast Field) {
        // todo: handle unnamed fields (tuple structs)
        self.field_name_idents.push(node.ident.clone().unwrap());
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
    let mut visitor = DeriveVisitor {
        field_name_idents: vec![],
        handle_type_idents: vec![],
    };
    visitor.visit_derive_input(&ast);

    let controller_ident = Ident::new(&format!("{}Controller", rack_ident), Span::call_site());
    let field_name_idents = visitor.field_name_idents;
    let handle_type_idents = visitor.handle_type_idents;

    // Generate the effect rack controller and the associated implementation
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
