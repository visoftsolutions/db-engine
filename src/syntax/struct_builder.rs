use proc_macro2::{Span, TokenStream, Ident};
use quote::{format_ident, quote};

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

    pub fn add_field(&mut self, name: &str, field_type: &str) -> &mut Self {
        self.fields.push((name.to_string(), field_type.to_string()));
        self
    }

    fn name_iden(&self) -> Ident {
        syn::Ident::new(&&self.name, Span::call_site())
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name_iden = self.name_iden();
        let field_defs: Vec<_> = self
            .fields
            .iter()
            .map(|(field_name, field_type)| {
                let name_iden = format_ident!("{}", field_name);
                let type_iden = format_ident!("{}", field_type);
                quote! { pub #name_iden: #type_iden }
            })
            .collect();

        quote! {
            #[derive(Debug, Serialize, Deserialize)]
            pub struct #name_iden {
                #(#field_defs,)*
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code() {
        let tokens = StructSyntaxBuilder::new("Person")
            .add_field("name", "String")
            .add_field("age", "i32")
            .to_tokens();
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());
        assert_eq!(
            code,
            "struct Person {\n    name: String,\n    age: i32,\n}\n"
        );
    }
}
