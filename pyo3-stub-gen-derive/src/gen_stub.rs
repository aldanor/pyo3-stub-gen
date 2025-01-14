//! Code generation for embedding metadata for generating Python stub file.
//!
//! These metadata are embedded as `inventory::submit!` block like:
//!
//! ```rust
//! # use pyo3::*;
//! # use pyo3_stub_gen::type_info::*;
//! # struct PyPlaceholder;
//! inventory::submit!{
//!     PyClassInfo {
//!         pyclass_name: "Placeholder",
//!         module: Some("my_module"),
//!         struct_id: std::any::TypeId::of::<PyPlaceholder>,
//!         members: &[
//!             MemberInfo {
//!                 name: "name",
//!                 r#type: <String as IntoPy<PyObject>>::type_output,
//!             },
//!             MemberInfo {
//!                 name: "ndim",
//!                 r#type: <usize as IntoPy<PyObject>>::type_output,
//!             },
//!             MemberInfo {
//!                 name: "description",
//!                 r#type: <Option<String> as IntoPy<PyObject>>::type_output,
//!             },
//!         ],
//!         doc: "",
//!     }
//! }
//! ```
//!
//! and this submodule responsible for generating such codes from Rust code like
//!
//! ```rust
//! # use pyo3::*;
//! #[pyclass(mapping, module = "my_module", name = "Placeholder")]
//! #[derive(Debug, Clone)]
//! pub struct PyPlaceholder {
//!     #[pyo3(get)]
//!     pub name: String,
//!     #[pyo3(get)]
//!     pub ndim: usize,
//!     #[pyo3(get)]
//!     pub description: Option<String>,
//!     pub custom_latex: Option<String>,
//! }
//! ```
//!
//! Mechanism
//! ----------
//! Code generation will take three steps:
//!
//! 1. Parse input [proc_macro2::TokenStream] into corresponding syntax tree component in [syn],
//!    - e.g. [ItemStruct] for `#[pyclass]`, [ItemImpl] for `#[pymethods]`, and so on.
//! 2. Convert syntax tree components into `*Info` struct using [TryInto].
//!    - e.g. [PyClassInfo] is converted from [ItemStruct], [PyMethodsInfo] is converted from [ItemImpl], and so on.
//! 3. Generate token streams using implementation of [quote::ToTokens] trait for `*Info` structs.
//!    - [quote::quote!] macro uses this trait.
//!

mod arg;
mod attr;
mod member;
mod method;
mod new;
mod pyclass;
mod pyclass_enum;
mod pyfunction;
mod pymethods;
mod signature;
mod util;

use arg::*;
use attr::*;
use member::*;
use method::*;
use new::*;
use pyclass::*;
use pyclass_enum::*;
use pyfunction::*;
use pymethods::*;
use signature::*;
use util::*;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, ItemEnum, ItemFn, ItemImpl, ItemStruct, Result};

pub fn pyclass(item: TokenStream2) -> Result<TokenStream2> {
    let inner = PyClassInfo::try_from(parse2::<ItemStruct>(item.clone())?)?;
    Ok(quote! {
        #item
        pyo3_stub_gen::inventory::submit! {
            #inner
        }
    })
}

pub fn pyclass_enum(item: TokenStream2) -> Result<TokenStream2> {
    let inner = PyEnumInfo::try_from(parse2::<ItemEnum>(item.clone())?)?;
    Ok(quote! {
        #item
        pyo3_stub_gen::inventory::submit! {
            #inner
        }
    })
}

pub fn pymethods(item: TokenStream2) -> Result<TokenStream2> {
    let inner = PyMethodsInfo::try_from(parse2::<ItemImpl>(item.clone())?)?;
    Ok(quote! {
        #item
        pyo3_stub_gen::inventory::submit! {
            #inner
        }
    })
}

pub fn pyfunction(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2> {
    let mut inner = PyFunctionInfo::try_from(parse2::<ItemFn>(item.clone())?)?;
    inner.parse_attr(attr)?;
    Ok(quote! {
        #item
        pyo3_stub_gen::inventory::submit! {
            #inner
        }
    })
}
