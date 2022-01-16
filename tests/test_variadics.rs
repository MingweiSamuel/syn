#[macro_use]
mod macros;

use quote::quote;
use syn::{Item, Type};

#[test]
fn test_variadic_type() {
    let tokens = quote! { ...T };

    snapshot!(tokens as Type, @r###"
    Type::Variadic {
        elem: Type::Path {
            path: Path {
                segments: [
                    PathSegment {
                        ident: "T",
                        arguments: None,
                    },
                ],
            },
        },
    }
    "###);
}

#[test]
fn test_c_variadic() {
    let tokens = quote! {
        fn f(...) {
        }
    };

    snapshot!(tokens as Item, @r###"
    Item::Fn {
        vis: Inherited,
        sig: Signature {
            ident: "f",
            generics: Generics,
            variadic: Some(Variadic),
            output: Default,
        },
        block: Block,
    }
    "###);
}

#[test]
fn test_variadic_generic_args() {
    let tokens = quote! {
        fn f<...Ts>(items: ...Ts) {
        }
    };

    snapshot!(tokens as Item, @r###"
    Item::Fn {
        vis: Inherited,
        sig: Signature {
            ident: "f",
            generics: Generics {
                lt_token: Some,
                params: [
                    Type(TypeParam {
                        ellipses_token: Some,
                        ident: "Ts",
                    }),
                ],
                gt_token: Some,
            },
            inputs: [
                Typed(PatType {
                    pat: Pat::Ident {
                        ident: "items",
                    },
                    ty: Type::Variadic {
                        elem: Type::Path {
                            path: Path {
                                segments: [
                                    PathSegment {
                                        ident: "Ts",
                                        arguments: None,
                                    },
                                ],
                            },
                        },
                    },
                }),
            ],
            output: Default,
        },
        block: Block,
    }
    "###);
}
