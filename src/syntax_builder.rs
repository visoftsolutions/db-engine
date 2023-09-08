use proc_macro2::Span;
use quote::{quote, format_ident};

pub struct StructSyntaxBuilder {
    name: String,
    fields: Vec<(String, String)>,
}

impl StructSyntaxBuilder {
    pub fn new(name: &str) -> Self {
        StructSyntaxBuilder {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(mut self, name: &str, field_type: &str) -> Self {
        self.fields.push((name.to_string(), field_type.to_string()));
        self
    }

    pub fn to_string(&self) -> String {
        let name_iden = syn::Ident::new(&&self.name, Span::call_site());
        let field_defs: Vec<_> = self.fields.iter()
            .map(|(field_name, field_type)| {
                let name_iden = format_ident!("{}", field_name);
                let type_iden = format_ident!("{}", field_type);
                quote! { #name_iden: #type_iden }
            })
            .collect();

        let output = quote! {
            struct #name_iden {
                #(#field_defs,)*
            }
        };
        let syntax_tree = syn::parse2(output).unwrap();
        prettyplease::unparse(&syntax_tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code() {
        let code = StructSyntaxBuilder::new("Person")
            .add_field("name", "String")
            .add_field("age", "i32")
            .to_string();
        assert_eq!(code, "struct Person {\n    name: String,\n    age: i32,\n}\n");
    }
}
