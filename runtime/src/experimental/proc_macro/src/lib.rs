use std::collections::BTreeMap;
use cached::lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{Attribute, DeriveInput, Field, Fields, Variant};

struct GateDef {
    ident: String,
    struct_ident: String,
    size: usize,
    fields: Vec<String>,
    matrix: TokenStream,
    dagger: Dagger,
}

struct Dagger {
    pub gate_ident: String,
    pub fields: Option<TokenStream>,
}

struct GateDefImplCases {
    matrix: TokenStream,
    dagger: TokenStream,
}

impl GateDef {
    pub fn new(
        ident: String, struct_ident: String, size: usize,
        fields: Vec<String>, matrix: TokenStream, dagger: Dagger
    ) -> Self {
        Self { ident, struct_ident, size, fields, matrix, dagger }
    }
}

#[proc_macro_derive(GateDef, attributes(mat, dagger, params))]
pub fn derive_gates_def_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse::<DeriveInput>(input).expect("Failed to parse macro input");
    // let ident = ast.ident.to_string().parse::<TokenStream>().unwrap();
    if let syn::Data::Enum(data) = ast.data {
        let gate_defs = data.variants.iter()
            .map(parse_gate_def)
            .collect::<syn::Result<Vec<GateDef>>>();
        let gate_defs: Vec<GateDef> = match gate_defs {
            Ok(gate_defs) => gate_defs,
            Err(err) => panic!("{}", err.to_string()),
        };

        let gate_struct_ident_map: BTreeMap<&String, &String> = gate_defs.iter()
            .map(|gate_def| (&gate_def.ident, &gate_def.struct_ident))
            .collect();

        let gate_def_impls = gate_defs.iter()
            .map(|gate_def| {
                let gate_struct_ident: TokenStream = gate_def.struct_ident.parse().unwrap();
                let gate_fields: TokenStream = gate_def.fields.iter().map(|field| {
                    let field_ident: TokenStream = field.parse().unwrap();
                    quote!(#field_ident: f64,)
                }).collect::<TokenStream>();
                let dagger_struct_ident: TokenStream = gate_struct_ident_map[&gate_def.ident].parse().unwrap();
                let dagger_fields = if let Some(fields) = &gate_def.dagger.fields {
                    quote!(target: self.target, #fields)
                } else {
                    quote!(target: self.target)
                };

                let gate_size = gate_def.size;
                let mat_size = 2usize.pow(gate_size as u32);

                let mat_type: TokenStream = format!(
                    "nalgebra::SMatrix<num::complex::Complex64, {0}, {0}>",
                    2u32.pow(gate_size as u32),
                ).parse().unwrap();
                let mat = gate_def.matrix.clone();

                let quoted_gate_ident_str: TokenStream = format!("\"{}\"", gate_def.ident).parse().unwrap();

                quote! {
                    #[derive(Copy, Clone, Debug, PartialEq)]
                    pub struct #gate_struct_ident {
                        target: [QubitAddr; #gate_size],
                        #gate_fields
                    }

                    impl_single_gate!(#gate_struct_ident: #quoted_gate_ident_str);

                    impl Operation for #gate_struct_ident {
                        fn map_qubits(
                            &self, f: &dyn Fn(QubitAddr) -> QubitAddr
                        ) -> Self where Self: Sized {
                            todo!()
                        }

                        fn size(&self) -> usize {
                            self.target.len()
                        }
                    }

                    impl Dagger<#dagger_struct_ident> for #gate_struct_ident {
                        fn dagger(self) -> Self {
                            #dagger_struct_ident { #dagger_fields }
                        }
                    }

                    impl StaticOperation<nalgebra::Const<#mat_size>> for #gate_struct_ident { }

                    impl ToMat<nalgebra::Const<#mat_size>> for #gate_struct_ident {
                        fn to_mat(&self) -> #mat_type {
                            #mat
                        }
                    }
                }
            }).collect::<Vec<TokenStream>>();

        gate_def_impls.into_iter().collect::<TokenStream>().into()
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

    let params_attr: Option<&Attribute> = gate_def.attrs.iter()
        .find(|attr| attr.path.is_ident("params"));
    let fields = params_attr.map(parse_attr_params).unwrap_or(Vec::<String>::new());

    let struct_ident = match &gate_def.fields {
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                return Err(syn::Error::new_spanned(
                    gate_def.fields.to_token_stream(),
                    "Gate variant should only have one binding struct"
                ))
            } else {
                fields.unnamed.first().unwrap().to_token_stream().to_string()
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                gate_def.fields.to_token_stream(),
                "Invalid gate variant format"
            ))
        }
    };

    Ok(GateDef::new(gate_def.ident.to_string(), struct_ident, size, fields, mat, dagger))
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
            gate_ident: dagger_text.parse().unwrap(),
            fields: None,
        }
    } else {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new("([A-Za-z0-9]+)\\s*(\\{.*\\})?").unwrap();
        }
        let dagger_text = dagger_text.replace("\n", " ");
        if let Some(cap) = PATTERN.captures_iter(&dagger_text).next() {
            let gate_ident = cap.get(1).unwrap().as_str().to_string();
            if let Some(fields) = cap.get(2) {
                Dagger {
                    gate_ident,
                    fields: Some(
                        fields.as_str()
                            .trim_start_matches('{')
                            .trim_end_matches('}')
                            .parse().unwrap()
                    ),
                }
            } else {
                Dagger {
                    gate_ident,
                    fields: None,
                }
            }
        } else {
            panic!("Invalid dagger syntax: {}", dagger_text);
        }
    }
}

fn parse_attr_params(dagger_attr: &Attribute) -> Vec<String> {
    let params_text = dagger_attr.tokens.to_string();
    let params_text = params_text
        .trim_start_matches(['(', ' '])
        .trim_end_matches(&[')', ' ']);
    if params_text.chars().all(char::is_alphabetic) {
        vec![params_text.to_string()]
    } else {
        params_text.split(',').map(|s| s.trim().to_string()).collect()
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
    rule alpha() -> String
        = "α" { String::from("num::complex::Complex64::from(self.alpha as f64)") }
    rule beta() -> String
        = "β" { String::from("num::complex::Complex64::from(self.beta as f64)") }
    rule theta() -> String
        = "θ" { String::from("num::complex::Complex64::from(self.theta as f64)") }
    rule phi() -> String
        = "φ" { String::from("num::complex::Complex64::from(self.phi as f64)") }
    rule lambda() -> String
        = "λ" { String::from("num::complex::Complex64::from(self.lambda as f64)") }
    rule var() -> String
        = ident: $([ 'a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) {
            format!("num::complex::Complex64::from(self.{} as f64)", ident)
        }
        / alpha()
        / beta()
        / lambda()
        / theta()
        / phi()
    rule _ -> ()
	  = [' ' | '\n' | '\t' | '\r']* { }
});
