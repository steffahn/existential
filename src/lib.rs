#![no_std]
// reasonable clippy categories
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable clippy::restriction lints
#![warn(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
// reasonable rustc lints
#![warn(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
// reasonable rustdoc lints
#![warn(
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags
)]

//! [![crates.io]](https://crates.io/crates/existential)
//! [![github]](https://github.com/steffahn/existential)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/existential#License)
//!
//! [Existential types](https://wiki.haskell.org/Existential_type) in Rust, offering existential
//! quantification over lifetimes, but as a library. This works because Rust has
//! [parametricity](https://en.wikipedia.org/wiki/Parametricity) for generic lifetime arguments.
//! 
//! _TODO: this crate is still undocumented._
//!
//! [github]: https://img.shields.io/badge/github-steffahn/existential-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/existential.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/existential.svg
//! [docs.rs]: https://docs.rs/existential/badge.svg

use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
};

pub trait TyConFor<'a> {
    type Applied;
}

pub type Apply<'a, C> = <C as TyConFor<'a>>::Applied;

pub trait TyCon: for<'a> TyConFor<'a> {}
impl<C: ?Sized> TyCon for C where C: for<'a> TyConFor<'a> {}

pub struct Existential<'lower_bound, C>
where
    C: TyCon,
{
    marker: PhantomData<&'lower_bound ()>,
    inner: Apply<'static, C>,
}

impl<'lower_bound, C> Existential<'lower_bound, C>
where
    C: TyCon,
{
    pub fn new<'a: 'lower_bound>(inner: Apply<'a, C>) -> Existential<'lower_bound, C> {
        let inner = ManuallyDrop::new(inner);
        unsafe {
            Self {
                marker: PhantomData,
                inner: mem::transmute_copy::<Apply<'a, C>, Apply<'static, C>>(&inner),
            }
        }
    }
    pub fn with<'s, F, O>(&'s self, f: F) -> O
    where
        F: for<'a> FnOnce(&'s Apply<'a, C>, PhantomData<&'lower_bound &'a ()>) -> O,
    {
        f(&self.inner, PhantomData)
    }
    pub fn with_mut<'s, F, O>(&'s mut self, f: F) -> O
    where
        F: for<'a> FnOnce(&'s mut Apply<'a, C>, PhantomData<&'lower_bound &'a ()>) -> O,
    {
        f(&mut self.inner, PhantomData)
    }
    pub fn with_owned<F, O>(self, f: F) -> O
    where
        F: for<'a> FnOnce(Apply<'a, C>, PhantomData<&'lower_bound &'a ()>) -> O,
    {
        f(self.inner, PhantomData)
    }
}
