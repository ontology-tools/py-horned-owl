//! Attribute macros that turn plain Rust declarations of py-horned-owl's
//! model into Python classes.
//!
//! ```ignore
//! #[model(variant_of = ClassExpression)]
//! pub struct ObjectSomeValuesFrom {
//!     #[model(horned = ope)]
//!     pub object_property_expression: ObjectPropertyExpression,
//!     #[model(horned = bce)]
//!     pub class_expression: BoxWrap<ClassExpression>,
//! }
//! ```
//!
//! The macros generate the class definition, the constructor, the
//! conversions to and from horned-owl and one-line forwarding methods. The
//! behaviour shared by all classes (`__repr__`, pickling, `copy.replace`, ...)
//! lives in ordinary functions in py-horned-owl's `src/model/protocol.rs`.
//!
//! # `#[model(...)]` on a struct
//!
//! * `variant_of = Enum` or `variant_of = Enum::Variant`: the class is a
//!   variant of a horned-owl enum (default variant: the struct's name). Its
//!   Python base class is the enum's (`<Enum>Base`, see `#[model_enum]`).
//! * `horned = path::Type`: the horned-owl struct the class wraps, for classes
//!   that are not enum variants. Default: `horned_owl::model::<Name>`.
//! * `extends = Base`: the Python base class, for classes that are not enum
//!   variants but still belong to a hierarchy (`Class` is a `ClassExpression`).
//! * `render = direct` or `render = functional`: generate `serialize` and
//!   `__str__` (functional syntax), for classes without a base class (which
//!   has them). `functional` for classes horned-owl cannot write in
//!   Manchester syntax.
//! * `str = field`: `__str__` is that field's `str()`.
//! * `entity`: a named entity, holding one field `iri: IRI`. `str()` is the
//!   IRI and a `str` converts to the entity (see `entity_from_str`).
//! * `coerce = function`, `accepts = [Type, ...]`: other Python values the
//!   class is extracted from. `function(&Bound<PyAny>) -> PyResult<Option<T>>`
//!   with `T: Into<Self>`; `accepts` lists their types for the stubs.
//! * `skip(new, repr, reduce, replace, match_args)`: leave out generated
//!   methods, to write them by hand.
//!
//! # `#[model(...)]` on a field
//!
//! * `horned = name` or `horned = 0`: the horned-owl field (by name, or by
//!   position in a tuple struct or variant) when the Python name differs.
//! * `default = expr`: a default for the constructor argument.
//!
//! # `#[model_enum(...)]` on an enum
//!
//! An enum whose variants are named like horned-owl's and each hold one model
//! class. Generates a field-less Python base class `<Enum>Base`, named like
//! the enum in Python, with `serialize` and `__str__` for all its subclasses.
//! Mark variants holding a class that is not a `variant_of` the enum with
//! `#[model(wraps)]` (`ClassExpression::Class(Class)`), or all of them with
//! `#[model_enum(wraps)]` (`Component`). Such a class `extends` the base.
//! Takes `coerce` and `accepts` like a struct.
//!
//! # `#[model_union]` on an enum
//!
//! An enum whose variants each hold one Python type, named like horned-owl's.
//! It has no Python class: in Python it is the union of the variants' types,
//! which are tried in order.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Attribute, Expr, Fields, Ident, ItemEnum, ItemStruct, LitStr, Member, Path,
    Token, Type,
};

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    model_struct(attr.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn model_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    model_enum_impl(attr.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn model_union(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    model_union_impl(attr.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Other Python values a class or enum is extracted from.
#[derive(Default)]
struct Coercion {
    function: Option<Path>,
    accepts: Vec<Type>,
}

#[derive(Default)]
struct StructOptions {
    variant_of: Option<Path>,
    horned: Option<Path>,
    extends: Option<Path>,
    render: Option<Ident>,
    str_field: Option<Ident>,
    entity: bool,
    coercion: Coercion,
    skip: Vec<Ident>,
}

impl StructOptions {
    fn skips(&self, method: &str) -> bool {
        self.skip.iter().any(|s| s == method)
    }
}

/// Parses `coerce = ...` and `accepts = [...]`; `false` for other options.
fn parse_coercion(coercion: &mut Coercion, meta: &syn::meta::ParseNestedMeta) -> syn::Result<bool> {
    if meta.path.is_ident("coerce") {
        coercion.function = Some(meta.value()?.parse()?);
    } else if meta.path.is_ident("accepts") {
        let value = meta.value()?;
        let content;
        syn::bracketed!(content in value);
        coercion.accepts = Punctuated::<Type, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
    } else {
        return Ok(false);
    }
    Ok(true)
}

fn parse_struct_options(attr: TokenStream2) -> syn::Result<StructOptions> {
    let mut options = StructOptions::default();
    syn::meta::parser(|meta| {
        if parse_coercion(&mut options.coercion, &meta)? {
        } else if meta.path.is_ident("variant_of") {
            options.variant_of.replace(meta.value()?.parse()?);
        } else if meta.path.is_ident("horned") {
            options.horned.replace(meta.value()?.parse()?);
        } else if meta.path.is_ident("extends") {
            options.extends.replace(meta.value()?.parse()?);
        } else if meta.path.is_ident("render") {
            let render: Ident = meta.value()?.parse()?;
            if render != "direct" && render != "functional" {
                return Err(syn::Error::new(
                    render.span(),
                    "expected `direct` or `functional`",
                ));
            }
            options.render = Some(render);
        } else if meta.path.is_ident("str") {
            options.str_field = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("entity") {
            options.entity = true;
        } else if meta.path.is_ident("skip") {
            meta.parse_nested_meta(|m| {
                options.skip.push(m.path.require_ident()?.clone());
                Ok(())
            })?;
        } else {
            return Err(meta.error("unknown `model` option"));
        }
        Ok(())
    })
    .parse2(attr)?;
    Ok(options)
}

struct Field {
    ident: Ident,
    ty: Type,
    /// The horned-owl field: a name, or a position in a tuple.
    horned: Member,
    default: Option<Expr>,
}

/// Reads the `#[model(...)]` attributes of the fields, removes them and makes
/// the fields readable from Python.
fn take_fields(item: &mut ItemStruct) -> syn::Result<Vec<Field>> {
    let Fields::Named(named) = &mut item.fields else {
        return Err(syn::Error::new_spanned(
            &item.ident,
            "`#[model]` needs a struct with named fields",
        ));
    };
    named
        .named
        .iter_mut()
        .map(|f| {
            let ident = f.ident.clone().unwrap();
            let mut field = Field {
                horned: Member::Named(ident.clone()),
                ident,
                ty: f.ty.clone(),
                default: None,
            };
            for attr in take_model_attrs(&mut f.attrs) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("horned") {
                        field.horned = meta.value()?.parse()?;
                    } else if meta.path.is_ident("default") {
                        field.default = Some(meta.value()?.parse()?);
                    } else {
                        return Err(meta.error("unknown `model` field option"));
                    }
                    Ok(())
                })?;
            }
            // Read-only: the classes are frozen.
            f.attrs.push(syn::parse_quote!(#[pyo3(get)]));
            Ok(field)
        })
        .collect()
}

fn take_model_attrs(attrs: &mut Vec<Attribute>) -> Vec<Attribute> {
    let (model, rest) = std::mem::take(attrs)
        .into_iter()
        .partition(|a| a.path().is_ident("model"));
    *attrs = rest;
    model
}

/// `#[doc = doc!(Name)]` (the horned-owl documentation) unless the item has
/// documentation of its own.
fn default_doc(attrs: &[Attribute], name: &Ident) -> TokenStream2 {
    if attrs.iter().any(|a| a.path().is_ident("doc")) {
        quote!()
    } else {
        quote!(#[doc = doc!(#name)])
    }
}

fn horned(ty: impl ToTokens) -> TokenStream2 {
    quote!(::horned_owl::model::#ty<::horned_owl::model::ArcStr>)
}

/// A horned-owl value built from the fields of `model` (a `&Self`
/// expression): `path(a, b)` for a tuple, `path { x: a, y: b }` otherwise.
fn horned_constructor(
    path: &TokenStream2,
    model: TokenStream2,
    fields: &[Field],
) -> syn::Result<TokenStream2> {
    let value = |f: &Field| {
        let ident = &f.ident;
        quote!(crate::wrappers::IntoCompatible::into_c(&#model.#ident))
    };
    Ok(if is_tuple(fields)? {
        let values = by_position(fields).into_iter().map(value);
        quote!(#path(#(#values),*))
    } else {
        let names = fields.iter().map(|f| &f.horned);
        let values = fields.iter().map(value);
        quote!(#path { #(#names: #values),* })
    })
}

/// A pattern binding a horned-owl value's fields to the Python field names.
fn horned_pattern(path: &TokenStream2, fields: &[Field]) -> syn::Result<TokenStream2> {
    Ok(if is_tuple(fields)? {
        let idents = by_position(fields).into_iter().map(|f| &f.ident);
        quote!(#path(#(#idents),*))
    } else {
        let bindings = fields.iter().map(|f| {
            let ident = &f.ident;
            match &f.horned {
                Member::Named(name) if name == ident => quote!(#ident),
                name => quote!(#name: #ident),
            }
        });
        quote!(#path { #(#bindings),* })
    })
}

/// `Self` built from the bindings of [`horned_pattern`].
fn from_bindings(fields: &[Field]) -> TokenStream2 {
    let idents = fields.iter().map(|f| &f.ident);
    let values = fields.iter().map(|f| {
        let (ident, ty) = (&f.ident, &f.ty);
        quote!(crate::wrappers::IntoCompatible::<#ty>::into_c(#ident))
    });
    quote!(Self { #(#idents: #values),* })
}

fn is_tuple(fields: &[Field]) -> syn::Result<bool> {
    let tuple = matches!(fields[0].horned, Member::Unnamed(_));
    match fields
        .iter()
        .find(|f| matches!(f.horned, Member::Unnamed(_)) != tuple)
    {
        Some(f) => Err(syn::Error::new_spanned(
            &f.ident,
            "either all or no fields need a position (`horned = 0`)",
        )),
        None => Ok(tuple),
    }
}

fn by_position(fields: &[Field]) -> Vec<&Field> {
    let mut fields: Vec<&Field> = fields.iter().collect();
    fields.sort_by_key(|f| match &f.horned {
        Member::Unnamed(i) => i.index,
        Member::Named(_) => unreachable!(),
    });
    fields
}

/// `left | right | ...` as a PyO3 type hint.
fn hint_union(hints: Vec<TokenStream2>) -> TokenStream2 {
    hints
        .into_iter()
        .rev()
        .reduce(|right, left| {
            quote!(::pyo3::inspect::PyStaticExpr::BinOp {
                left: &#left,
                op: ::pyo3::inspect::PyStaticOperator::BitOr,
                right: &#right,
            })
        })
        .unwrap()
}

/// `FromPyObject` that runs `casts` (returning on a class of the right type)
/// and then the coercion function. `hint` is the stub type `casts` accepts.
fn from_py_object(
    name: &Ident,
    hint: TokenStream2,
    casts: TokenStream2,
    coercion: &Coercion,
) -> TokenStream2 {
    let mut hints = vec![hint];
    hints.extend(
        coercion
            .accepts
            .iter()
            .map(|ty| quote!(<#ty as ::pyo3::FromPyObject<'static, 'static>>::INPUT_TYPE)),
    );
    let hint = hint_union(hints);
    let coerce = coercion.function.as_ref().map(|f| {
        quote! {
            if let Some(value) = #f(&ob)? {
                return Ok(::core::convert::Into::into(value));
            }
        }
    });
    quote! {
        impl<'py> ::pyo3::FromPyObject<'_, 'py> for #name {
            const INPUT_TYPE: ::pyo3::inspect::PyStaticExpr = #hint;
            type Error = ::pyo3::PyErr;

            fn extract(ob: ::pyo3::Borrowed<'_, 'py, ::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
                use ::pyo3::types::{PyAnyMethods, PyTypeMethods};
                #casts
                #coerce
                Err(::pyo3::exceptions::PyTypeError::new_err(format!(
                    "expected {}, got {}",
                    stringify!(#name),
                    ob.get_type().name()?
                )))
            }
        }
    }
}

/// `serialize`, rendering the horned-owl value `to_horned` (an expression
/// over `slf: &Bound<Self>`), in Manchester syntax too unless `functional`.
fn serialize_method(name: &Ident, to_horned: TokenStream2, functional: bool) -> TokenStream2 {
    let (doc, syntaxes, manchester) = if functional {
        (
            quote! {
                /// Renders this element on its own, in OWL functional (`"ofn"`)
                /// syntax. The per-element counterpart to
                /// `PyIndexedOntology.save_to_string`.
                ///
                /// horned-owl has no Manchester rendering for this class:
                /// Manchester syntax writes it only inside the element that
                /// contains it, so `"omn"` is an error.
            },
            quote!(FunctionalSyntax),
            quote!(return Err(crate::snippet::no_manchester_rendering(stringify!(#name)))),
        )
    } else {
        (
            quote! {
                /// Renders this element on its own, in OWL functional (`"ofn"`,
                /// the default) or OWL 2 Manchester (`"omn"`) syntax. The
                /// per-element counterpart to `PyIndexedOntology.save_to_string`.
            },
            quote!(SnippetSyntaxes),
            quote!(crate::as_omn!(value, prefix_mapping)),
        )
    };
    quote! {
        #[::pyo3::pymethods]
        impl #name {
            #doc
            #[pyo3(signature = (serialization = None, prefix_mapping = None))]
            fn serialize(
                slf: &::pyo3::Bound<'_, Self>,
                serialization: Option<crate::wrappers::LiteralStr<crate::wrappers::#syntaxes>>,
                prefix_mapping: Option<&crate::prefix_mapping::PrefixMapping>,
            ) -> ::pyo3::PyResult<String> {
                let value = #to_horned;
                Ok(match crate::snippet::parse_syntax(serialization.as_deref().unwrap_or("ofn"))? {
                    crate::snippet::SnippetSyntax::Manchester => #manchester,
                    crate::snippet::SnippetSyntax::Functional => crate::as_ofn!(value, prefix_mapping),
                })
            }
        }
    }
}

fn str_as_functional(name: &Ident) -> TokenStream2 {
    quote! {
        #[::pyo3::pymethods]
        impl #name {
            fn __str__(slf: &::pyo3::Bound<'_, Self>) -> ::pyo3::PyResult<String> {
                Self::serialize(slf, None, None)
            }
        }
    }
}

fn model_struct(attr: TokenStream2, mut item: ItemStruct) -> syn::Result<TokenStream2> {
    let mut options = parse_struct_options(attr)?;
    let fields = take_fields(&mut item)?;
    let name = item.ident.clone();
    let doc = default_doc(&item.attrs, &name);
    let protocol = quote!(crate::model::protocol);

    if options.entity {
        if fields.len() != 1 || fields[0].ident != "iri" {
            return Err(syn::Error::new_spanned(
                &name,
                "an `entity` has exactly one field, `iri`",
            ));
        }
        if options.coercion.function.is_some() {
            return Err(syn::Error::new_spanned(
                &name,
                "an `entity` converts from `str`, it takes no `coerce`",
            ));
        }
        options.coercion.function =
            Some(syn::parse_quote!(crate::model::protocol::entity_from_str::<#name>));
        options.coercion.accepts.push(syn::parse_quote!(String));
        options
            .str_field
            .get_or_insert_with(|| format_ident!("iri"));
    }

    // `variant_of = Enum` or `variant_of = Enum::Variant`.
    let variant = match &options.variant_of {
        Some(path) => {
            let segments: Vec<&Ident> = path.segments.iter().map(|s| &s.ident).collect();
            match segments[..] {
                [e] => Some((e.clone(), name.clone())),
                [e, v] => Some((e.clone(), v.clone())),
                _ => {
                    return Err(syn::Error::new_spanned(
                        path,
                        "expected `Enum` or `Enum::Variant`",
                    ))
                }
            }
        }
        None => None,
    };
    let base = match (&options.extends, &variant) {
        (Some(base), _) => Some(base.to_token_stream()),
        (None, Some((e, _))) => Some(format_ident!("{}Base", e).to_token_stream()),
        (None, None) => None,
    };
    let horned_path = match &options.horned {
        Some(p) => p.to_token_stream(),
        None => quote!(::horned_owl::model::#name),
    };
    let horned_type = quote!(#horned_path<::horned_owl::model::ArcStr>);

    let idents: Vec<&Ident> = fields.iter().map(|f| &f.ident).collect();
    let names: Vec<String> = idents.iter().map(|i| i.to_string()).collect();

    let mut methods = Vec::new();
    if !options.skips("new") {
        let signature = fields.iter().map(|f| {
            let ident = &f.ident;
            match &f.default {
                Some(d) => quote!(#ident = #d),
                None => quote!(#ident),
            }
        });
        let parameters = fields.iter().map(|f| {
            let (ident, ty) = (&f.ident, &f.ty);
            quote!(#ident: #ty)
        });
        let construct = match &base {
            Some(base) => quote! {
                -> ::pyo3::PyClassInitializer<Self> {
                    ::pyo3::PyClassInitializer::from(#base).add_subclass(Self { #(#idents),* })
                }
            },
            None => quote!(-> Self { Self { #(#idents),* } }),
        };
        methods.push(quote! {
            #[new]
            #[pyo3(signature = (#(#signature),*))]
            fn new(#(#parameters),*) #construct
        });
    }
    if !options.skips("match_args") {
        methods.push(quote! {
            #[classattr]
            fn __match_args__(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<::pyo3::Bound<'_, ::pyo3::types::PyTuple>> {
                #protocol::match_args::<Self>(py)
            }
        });
    }
    if !options.skips("repr") {
        methods.push(quote! {
            fn __repr__(slf: &::pyo3::Bound<'_, Self>) -> ::pyo3::PyResult<String> {
                #protocol::repr(slf)
            }
        });
    }
    if !options.skips("reduce") {
        methods.push(quote! {
            fn __reduce__<'py>(
                slf: &::pyo3::Bound<'py, Self>,
            ) -> ::pyo3::PyResult<(::pyo3::Bound<'py, ::pyo3::types::PyType>, ::pyo3::Bound<'py, ::pyo3::types::PyTuple>)> {
                #protocol::reduce(slf)
            }
        });
    }
    if !options.skips("replace") {
        methods.push(quote! {
            /// Support for `copy.replace` (Python 3.13+): a copy with the
            /// given fields changed.
            #[pyo3(signature = (**changes))]
            fn __replace__<'py>(
                slf: &::pyo3::Bound<'py, Self>,
                changes: Option<&::pyo3::Bound<'py, ::pyo3::types::PyDict>>,
            ) -> ::pyo3::PyResult<Self> {
                #protocol::replace(slf, changes)
            }
        });
    }
    if let Some(field) = &options.str_field {
        methods.push(quote! {
            fn __str__(&self) -> String {
                self.#field.to_string()
            }
        });
    }

    // Conversions to and from horned-owl.
    let conversions = match &variant {
        Some((e, v)) => {
            let horned_enum = horned(e);
            let path = quote!(::horned_owl::model::#e::#v);
            let construct = horned_constructor(&path, quote!(self), &fields)?;
            let pattern = horned_pattern(&path, &fields)?;
            let from_bindings = from_bindings(&fields);
            quote! {
                impl #protocol::EnumVariant<#horned_enum> for #name {
                    fn to_horned(&self) -> #horned_enum {
                        #construct
                    }

                    fn from_horned(value: &#horned_enum) -> Self {
                        match value {
                            #pattern => #from_bindings,
                            _ => unreachable!(concat!("not a ", stringify!(#e), "::", stringify!(#v))),
                        }
                    }
                }
            }
        }
        None => {
            let construct = horned_constructor(&horned_path, quote!(value), &fields)?;
            let pattern = horned_pattern(&horned_path, &fields)?;
            let from_bindings = from_bindings(&fields);
            quote! {
                impl From<&#horned_type> for #name {
                    fn from(value: &#horned_type) -> Self {
                        let #pattern = value;
                        #from_bindings
                    }
                }

                impl From<&#name> for #horned_type {
                    fn from(value: &#name) -> Self {
                        #construct
                    }
                }

                conversions!(#name => #horned_type);
            }
        }
    };

    let from_iri = options.entity.then(|| {
        quote! {
            impl From<crate::model::IRI> for #name {
                fn from(iri: crate::model::IRI) -> Self {
                    Self { iri }
                }
            }
        }
    });

    let into_py_object = base.as_ref().map(|base| {
        quote! {
            // PyO3 only derives this for classes without a Python base class.
            impl<'py> ::pyo3::IntoPyObject<'py> for #name {
                const OUTPUT_TYPE: ::pyo3::inspect::PyStaticExpr = <#name as ::pyo3::PyTypeInfo>::TYPE_HINT;
                type Target = #name;
                type Output = ::pyo3::Bound<'py, #name>;
                type Error = ::pyo3::PyErr;

                fn into_pyobject(self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<Self::Output> {
                    ::pyo3::Bound::new(py, ::pyo3::PyClassInitializer::from(#base).add_subclass(self))
                }
            }
        }
    });

    let from_py = options.coercion.function.is_some().then(|| {
        from_py_object(
            &name,
            quote!(<#name as ::pyo3::PyTypeInfo>::TYPE_HINT),
            quote! {
                if let Ok(value) = ob.cast::<#name>() {
                    return Ok(value.get().clone());
                }
            },
            &options.coercion,
        )
    });

    let render = options.render.as_ref().map(|render| {
        let serialize = serialize_method(
            &name,
            quote!(<#horned_type>::from(slf.get())),
            render == "functional",
        );
        let str = options
            .str_field
            .is_none()
            .then(|| str_as_functional(&name));
        quote!(#serialize #str)
    });

    let pyclass_extends = base.as_ref().map(|b| quote!(extends = #b,));
    let pyclass_from_py = if options.coercion.function.is_some() {
        quote!(skip_from_py_object)
    } else {
        quote!(from_py_object)
    };

    Ok(quote! {
        #doc
        #[::pyo3::pyclass(module = "pyhornedowl.model", #pyclass_extends frozen, eq, ord, hash, #pyclass_from_py)]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #item

        #[::pyo3::pymethods]
        impl #name {
            #(#methods)*
        }

        impl #protocol::ModelFields for #name {
            const FIELDS: &'static [&'static str] = &[#(#names),*];

            fn field_values<'py>(&self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<Vec<::pyo3::Bound<'py, ::pyo3::PyAny>>> {
                use ::pyo3::IntoPyObjectExt;
                Ok(vec![#(self.#idents.clone().into_bound_py_any(py)?),*])
            }
        }

        #conversions
        #from_iri
        #into_py_object
        #from_py
        #render
    })
}

fn model_enum_impl(attr: TokenStream2, mut item: ItemEnum) -> syn::Result<TokenStream2> {
    let mut coercion = Coercion::default();
    let mut wrap_all = false;
    syn::meta::parser(|meta| {
        if parse_coercion(&mut coercion, &meta)? {
            Ok(())
        } else if meta.path.is_ident("wraps") {
            wrap_all = true;
            Ok(())
        } else {
            Err(meta.error("unknown `model_enum` option"))
        }
    })
    .parse2(attr)?;

    let name = item.ident.clone();
    let base = format_ident!("{}Base", name);
    let python_name = LitStr::new(&name.to_string(), name.span());
    let horned_enum = horned(&name);
    let doc = default_doc(&item.attrs, &name);
    let protocol = quote!(crate::model::protocol);

    let mut variants = Vec::new();
    let mut wraps = Vec::new();
    for v in &mut item.variants {
        let ty: Type = match &v.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => f.unnamed[0].ty.clone(),
            _ => {
                return Err(syn::Error::new_spanned(
                    &v.ident,
                    "expected a variant holding exactly one model class",
                ))
            }
        };
        for attr in take_model_attrs(&mut v.attrs) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("wraps") {
                    wraps.push((v.ident.clone(), ty.clone()));
                    Ok(())
                } else {
                    Err(meta.error("unknown `model` variant option"))
                }
            })?;
        }
        if wrap_all {
            wraps.push((v.ident.clone(), ty.clone()));
        }
        variants.push((v.ident.clone(), ty));
    }

    let variant_idents: Vec<&Ident> = variants.iter().map(|(v, _)| v).collect();
    let variant_types: Vec<&Type> = variants.iter().map(|(_, t)| t).collect();

    let wrap_impls = wraps.iter().map(|(v, ty)| {
        quote! {
            impl #protocol::EnumVariant<#horned_enum> for #ty {
                fn to_horned(&self) -> #horned_enum {
                    ::horned_owl::model::#name::#v(self.into())
                }

                fn from_horned(value: &#horned_enum) -> Self {
                    match value {
                        ::horned_owl::model::#name::#v(x) => x.into(),
                        _ => unreachable!(concat!("not a ", stringify!(#name), "::", stringify!(#v))),
                    }
                }
            }
        }
    });

    let serialize = serialize_method(
        &base,
        quote!(<#horned_enum>::from(&::pyo3::types::PyAnyMethods::extract::<#name>(slf.as_any())?)),
        false,
    );
    let str = str_as_functional(&base);

    let from_py = from_py_object(
        &name,
        quote!(<#base as ::pyo3::PyTypeInfo>::TYPE_HINT),
        quote! {
            #(
                if let Ok(value) = ob.cast::<#variant_types>() {
                    return Ok(#name::#variant_idents(value.get().clone()));
                }
            )*
        },
        &coercion,
    );

    Ok(quote! {
        #doc
        #[::pyo3::pyclass(name = #python_name, module = "pyhornedowl.model", subclass, frozen)]
        pub struct #base;

        #serialize
        #str

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #item

        #(
            impl From<#variant_types> for #name {
                fn from(value: #variant_types) -> Self {
                    #name::#variant_idents(value)
                }
            }
        )*

        #(#wrap_impls)*

        impl From<&#horned_enum> for #name {
            fn from(value: &#horned_enum) -> Self {
                use #protocol::EnumVariant;
                match value {
                    #( v @ ::horned_owl::model::#name::#variant_idents { .. } => #name::#variant_idents(EnumVariant::from_horned(v)), )*
                }
            }
        }

        impl From<&#name> for #horned_enum {
            fn from(value: &#name) -> Self {
                use #protocol::EnumVariant;
                match value {
                    #( #name::#variant_idents(v) => v.to_horned(), )*
                }
            }
        }

        conversions!(#name => #horned_enum);

        impl<'py> ::pyo3::IntoPyObject<'py> for #name {
            const OUTPUT_TYPE: ::pyo3::inspect::PyStaticExpr = <#base as ::pyo3::PyTypeInfo>::TYPE_HINT;
            type Target = ::pyo3::PyAny;
            type Output = ::pyo3::Bound<'py, ::pyo3::PyAny>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<Self::Output> {
                use ::pyo3::IntoPyObjectExt;
                match self {
                    #( #name::#variant_idents(v) => v.into_bound_py_any(py), )*
                }
            }
        }

        #from_py
    })
}

fn model_union_impl(attr: TokenStream2, item: ItemEnum) -> syn::Result<TokenStream2> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(
            attr,
            "`model_union` takes no options",
        ));
    }
    let name = item.ident.clone();
    let horned_enum = horned(&name);

    let mut variants = Vec::new();
    for v in &item.variants {
        match &v.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                variants.push((v.ident.clone(), f.unnamed[0].ty.clone()))
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    &v.ident,
                    "expected a variant holding exactly one type",
                ))
            }
        }
    }
    let idents: Vec<&Ident> = variants.iter().map(|(v, _)| v).collect();
    let types: Vec<&Type> = variants.iter().map(|(_, t)| t).collect();
    let output_type = hint_union(
        types
            .iter()
            .map(|ty| quote!(<#ty as ::pyo3::IntoPyObject<'static>>::OUTPUT_TYPE))
            .collect(),
    );
    let input_type = hint_union(
        types
            .iter()
            .map(|ty| quote!(<#ty as ::pyo3::FromPyObject<'static, 'static>>::INPUT_TYPE))
            .collect(),
    );

    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #item

        impl From<&#horned_enum> for #name {
            fn from(value: &#horned_enum) -> Self {
                match value {
                    #( ::horned_owl::model::#name::#idents(x) => #name::#idents(x.into()), )*
                }
            }
        }

        impl From<&#name> for #horned_enum {
            fn from(value: &#name) -> Self {
                match value {
                    #( #name::#idents(x) => ::horned_owl::model::#name::#idents(x.into()), )*
                }
            }
        }

        conversions!(#name => #horned_enum);

        impl<'py> ::pyo3::IntoPyObject<'py> for #name {
            const OUTPUT_TYPE: ::pyo3::inspect::PyStaticExpr = #output_type;
            type Target = ::pyo3::PyAny;
            type Output = ::pyo3::Bound<'py, ::pyo3::PyAny>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<Self::Output> {
                use ::pyo3::IntoPyObjectExt;
                match self {
                    #( #name::#idents(x) => x.into_bound_py_any(py), )*
                }
            }
        }

        impl<'py> ::pyo3::FromPyObject<'_, 'py> for #name {
            const INPUT_TYPE: ::pyo3::inspect::PyStaticExpr = #input_type;
            type Error = ::pyo3::PyErr;

            fn extract(ob: ::pyo3::Borrowed<'_, 'py, ::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
                use ::pyo3::types::{PyAnyMethods, PyTypeMethods};
                #(
                    if let Ok(x) = ob.extract::<#types>() {
                        return Ok(#name::#idents(x));
                    }
                )*
                Err(::pyo3::exceptions::PyTypeError::new_err(format!(
                    "expected {}, got {}",
                    stringify!(#name),
                    ob.get_type().name()?
                )))
            }
        }
    })
}
