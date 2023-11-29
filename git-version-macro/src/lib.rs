extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
/*
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};
*/
/*
use syn::{
	bracketed,
	parse::{Parse, ParseStream},
	parse_macro_input,
	punctuated::Punctuated,
	token::{Comma, Eq},
	Expr, Ident, LitStr,
};
*/

use git_version_core::{git_version_impl, Args};

/// Get the git version for the source code.
///
/// The following (named) arguments can be given:
///
/// - `args`: The arguments to call `git describe` with.
///   Default: `args = ["--always", "--dirty=-modified"]`
///
/// - `prefix`, `suffix`:
///   The git version will be prefixed/suffexed by these strings.
///
/// - `cargo_prefix`, `cargo_suffix`:
///   If either is given, Cargo's version (given by the CARGO_PKG_VERSION
///   environment variable) will be used if git fails instead of giving an
///   error. It will be prefixed/suffixed by the given strings.
///
/// - `fallback`:
///   If all else fails, this string will be given instead of reporting an
///   error.
///
/// # Examples
///
/// ```ignore
/// const VERSION: &str = git_version!();
/// ```
///
/// ```ignore
/// const VERSION: &str = git_version!(args = ["--abbrev=40", "--always"]);
/// ```
///
/// ```
/// # use git_version::git_version;
/// const VERSION: &str = git_version!(prefix = "git:", cargo_prefix = "cargo:", fallback = "unknown");
/// ```
#[proc_macro]
pub fn git_version(input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(input as Args);

	let tokens = match git_version_impl(args) {
		Ok(x) => x,
		Err(e) => e.to_compile_error(),
	};

	TokenStream::from(tokens)
}
