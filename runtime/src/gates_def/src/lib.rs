use cached::lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{Attribute, DeriveInput, Field, Fields, Variant};

struct GateDef {
    ident: String,
    size: usize,
    fields: Vec<Field>,
    matrix: TokenStream,
    dagger: Dagger,
}

struct Dagger {
    pub gate: TokenStream,
    pub fields: Option<TokenStream>,
}

struct GateDefImplCases {
    matrix: TokenStream,
    dagger: TokenStream,
}

impl GateDef {
    pub fn new(ident: String, size: usize, fields: Vec<Field>, matrix: TokenStream, dagger: Dagger) -> Self {
        Self { ident, size, fields, matrix, dagger }
    }

    pub fn into_impl_cases(self, ident: &TokenStream) -> GateDefImplCases {
        let gate_ident = self.ident.parse::<TokenStream>().unwrap();
        let gate_mat = self.matrix.to_token_stream();
        let dagger_ident = self.dagger.gate;
        let dagger_fields = self.dagger.fields;
        let gate_fields = self.fields.iter().map(|field| {
            let field_ident = field.ident.to_token_stream();
            quote! { #field_ident }
        }).collect::<Vec<TokenStream>>();
        GateDefImplCases {
            matrix: quote! {
                #ident::#gate_ident { #( # gate_fields),* } => { #gate_mat }
            },
            dagger: match dagger_fields {
                Some(fields) => quote! {
                    #ident::#gate_ident { #(#gate_fields),* } => {
                        #ident::#dagger_ident #fields
                    }
                },
                None => quote! {
                    #ident::#gate_ident { #(#gate_fields),* } => {
                        #ident::#dagger_ident { #(#gate_fields: *#gate_fields),* }
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(GatesDef, attributes(mat, dagger))]
pub fn derive_gates_def_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse::<DeriveInput>(input).expect("Failed to parse macro input");
    let ident = ast.ident.to_string().parse::<TokenStream>().unwrap();
    if let syn::Data::Enum(data) = ast.data {
        let gate_defs = data.variants.iter()
            .map(parse_gate_def)
            .collect::<syn::Result<Vec<GateDef>>>();
        let gate_defs: Vec<GateDef> = match gate_defs {
            Ok(gate_defs) => gate_defs,
            Err(err) => panic!("{}", err.to_string()),
        };
        let gate_size = &gate_defs[0].size.clone();
        for gate_def in &gate_defs {
            if &gate_def.size != gate_size {
                panic!("Gates size should be the same in one `GateDef` enum");
            }
        }
        let mat_type: TokenStream = format!(
            "nalgebra::SMatrix<num::complex::Complex64, {0}, {0}>",
            2u32.pow(*gate_size as u32),
        ).parse().unwrap();
        let gate_size = gate_size.to_token_stream();
        let cases: Vec<GateDefImplCases> = gate_defs.into_iter()
            .map(|gate_def| gate_def.into_impl_cases(&ident)).collect();
        let (mut cases_matrix, mut cases_dagger) = (vec![], vec![]);
        for case in cases {
            cases_matrix.push(case.matrix);
            cases_dagger.push(case.dagger);
        }
        quote!(
            impl #ident {
                // #[cached]
                pub fn get_matrix(&self) -> #mat_type {
                    match &self { #(#cases_matrix),* }
                }
                // #[cached]
                pub fn size(&self) -> usize { #gate_size }
                pub fn dagger(&self) -> Self {
                    match &self { #(#cases_dagger),* }
                }
            }
        ).into()
    } else {
        panic!("`#[derive(StdGates)]` only applicable to enums")
    }
}

fn parse_gate_def(gate_def: &Variant) -> syn::Result<GateDef> {
    let mat_attr: &Attribute = gate_def.attrs.iter()
        .find(|attr| attr.path.is_ident("mat"))
        .unwrap_or_else(|| panic!("Gate definition `{}` requires a `mat` attribute", gate_def.ident));
    let (mat, size) = match parse_attr_mat(mat_attr) {
        Ok(result) => result,
        Err(err) => return Err(syn::Error::new_spanned(
            &mat_attr.tokens,
            format!("Gate definition `{}`: {}", gate_def.ident, err)
        )),
    };
    let dagger_attr: &Attribute = gate_def.attrs.iter()
        .find(|attr| attr.path.is_ident("dagger"))
        .unwrap_or_else(|| panic!("Gate definition `{}` requires a `dagger` attribute", gate_def.ident));
    let dagger = parse_attr_dagger(dagger_attr);
    let fields = match &gate_def.fields {
        Fields::Named(fields) => fields.named.clone().into_iter().collect::<Vec<Field>>(),
        Fields::Unit => vec![],
        Fields::Unnamed(_) => return Err(syn::Error::new_spanned(
            gate_def.fields.to_token_stream(),
            "Fields of a gate definition must be named"
        )),
    };
    Ok(GateDef::new(gate_def.ident.to_string(), size, fields, mat, dagger))
}

fn parse_attr_mat(mat_attr: &Attribute) -> syn::Result<(TokenStream, usize)> {
    let mat = match complex_parser::mat(mat_attr.tokens.to_string().as_str()) {
        Ok(result) => result,
        Err(err) => return Err(syn::Error::new_spanned(
            &mat_attr.tokens,
            format!("Invalid matrix definition syntax: {}", err)
        )),
    };
    if !&mat.len().is_power_of_two() {
        return Err(syn::Error::new_spanned(&mat_attr.tokens, "Size of a quantum gate matrix must be a power of two"));
    }
    for row in mat.iter() {
        if row.len() != mat.len() {
            return Err(syn::Error::new_spanned(&mat_attr.tokens, "Invalid matrix definition"));
        }
    }
    let gate_size = (mat.len() as f64).log2() as usize;
    let rows: Vec<TokenStream> = mat.iter().map(|row| {
        let elems = row.iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<TokenStream>>();
        quote! { #(#elems),* }
    }).collect::<Vec<TokenStream>>();
    let mat_type: TokenStream = format!(
        "nalgebra::SMatrix::<num::complex::Complex64, {0}, {0}>", mat.len()
    ).parse().unwrap();
    Ok((quote! {
        #mat_type::from_vec(vec![ #(#rows),* ])
    }, gate_size))
}

fn parse_attr_dagger(dagger_attr: &Attribute) -> Dagger {
    let dagger_text = dagger_attr.tokens.to_string();
    let dagger_text = dagger_text
        .trim_start_matches(['(', ' '])
        .trim_end_matches(&[')', ' ']);
    if dagger_text.chars().all(char::is_alphabetic) {
        Dagger {
            gate: dagger_text.parse().unwrap(),
            fields: None,
        }
    } else {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new("([A-Za-z0-9]+)\\s*(\\{.*\\})").unwrap();
        }
        if let Some(cap) = PATTERN.captures_iter(dagger_text).next() {
            let gate_ident = cap.get(1).unwrap().as_str();
            let fields = cap.get(2).unwrap().as_str();
            Dagger {
                gate: gate_ident.parse().unwrap(),
                fields: Some(fields.parse().unwrap()),
            }
        } else {
            panic!("Invalid dagger syntax: {}", dagger_text);
        }
    }
}

peg::parser!(grammar complex_parser() for str {
    pub rule mat() -> Vec<Vec<String>>
        = "(" _ rows: (row() ** _) _ ")" { rows }
    rule row() -> Vec<String>
        = "|" _ exprs: (expr() ** ("," _)) _ "|" { exprs }
    pub rule expr() -> String
        = sum()
    rule sum() -> String
        = lhs: product() _ "+" _ rhs: product() { format!("{} + {}", lhs, rhs) }
        / lhs: product() _ "-" _ rhs: product() { format!("{} - {}", lhs, rhs) }
        / product()
    rule product() -> String
        = lhs: power() _ "*" _ rhs: power() { format!("{} * {}", lhs, rhs) }
        / lhs: power() _ "/" _ rhs: power() { format!("{} / {}", lhs, rhs) }
        / power()
    rule power() -> String
        = lhs: neg() _ "^" _ rhs: neg() { format!("num::pow::Pow::pow({}, {})", lhs, rhs) }
        / neg()
    rule neg() -> String
        = "-" _ value: complex() { format!("-{}", value) }
        / complex()
    rule complex() -> String
        = imaginary: float() im() { format!("({} * num::complex::Complex64::i())", imaginary) }
        / float()
        / im() { String::from("num::complex::Complex64::i()") }
        / "(" value: sum() ")" { format!("({})", value) }
        / sin() "(" value: sum() ")" { format!("num::complex::Complex64::sin({})", value) }
        / cos() "(" value: sum() ")" { format!("num::complex::Complex64::cos({})", value) }
        / sqrt() "(" value: sum() ")" { format!("num::complex::Complex64::sqrt({})", value) }
        / var()
    rule float() -> String
        = literal: $(['0'..='9']+("."['0'..='9']+)?) { format!("num::complex::Complex64::from({}f64)", literal) }
        / e()
        / pi()
    rule pi() -> String
        = "π" { String::from("num::complex::Complex64::from(std::f64::consts::PI)") }
    rule e() -> String
        = "e" { String::from("num::complex::Complex64::from(std::f64::consts::E)") }
    rule im() -> &'static str
        = "i" { "i" }
    rule sin() -> &'static str
        = "sin" { "sin" }
    rule cos() -> &'static str
        = "cos" { "cos" }
    rule sqrt() -> &'static str
        = "sqrt" { "sqrt" }
    rule var() -> String
        = ident: $([ 'a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) {
            format!("num::complex::Complex64::from(*{} as f64)", ident)
        }
    rule _ -> ()
	  = [' ' | '\n' | '\t' | '\r']* { }
});
