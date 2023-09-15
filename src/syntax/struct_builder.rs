use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

pub struct Field {
    name: String,
    field_type: String,
    decorators: Vec<String>,
}

impl Field {
    pub fn with_decorators(
        name: impl Into<String>,
        field_type: impl Into<String>,
        decorators: Vec<impl Into<String>>,
    ) -> Self {
        Field {
            name: name.into(),
            field_type: field_type.into(),
            decorators: decorators.into_iter().map(|d| d.into()).collect(),
        }
    }
    pub fn new(name: impl Into<String>, field_type: impl Into<String>) -> Self {
        Field::with_decorators(name, field_type, vec![] as Vec<String>)
    }
    pub fn to_tokens(&self) -> TokenStream {
        let name_iden = format_ident!("{}", self.name);
        // let type_iden = format_ident!("{}", self.field_type);
        let type_iden: TokenStream = syn::parse_str::<TokenStream>(&self.field_type)
            .unwrap()
            .into_token_stream();
        let decorators = &self
            .decorators
            .iter()
            .map(|d| d.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();
        quote! {
            #(#decorators)*
            pub #name_iden: #type_iden
        }
    }
}

pub struct StructSyntaxBuilder {
    name: String,
    fields: Vec<Field>,
}

impl StructSyntaxBuilder {
    pub fn new(name: &str) -> Self {
        StructSyntaxBuilder {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    fn name_iden(&self) -> Ident {
        syn::Ident::new(&self.name, Span::call_site())
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name_iden = self.name_iden();
        let field_defs: Vec<_> = self.fields.iter().map(Field::to_tokens).collect();

        quote! {
            #[derive(Debug, Serialize, Deserialize, Clone)]
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
            .add_field(Field::new("name", "String"))
            .add_field(Field::new("age", "i32"))
            .to_tokens();
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());
        assert_eq!(
            code,
            "struct Person {\n    name: String,\n    age: i32,\n}\n"
        );
    }
}
