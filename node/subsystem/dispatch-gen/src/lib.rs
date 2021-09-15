

// You should have received a copy of the GNU General Public License
// along with moonrabbit.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt;
use syn::{parse2, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, ItemEnum, Path, Result, Type, Variant};

#[proc_macro_attribute]
pub fn subsystem_dispatch_gen(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let attr: TokenStream = attr.into();
	let item: TokenStream = item.into();
	let mut backup = item.clone();
	impl_subsystem_dispatch_gen(attr.into(), item).unwrap_or_else(|err| {
		backup.extend(err.to_compile_error());
		backup
	}).into()
}

/// An enum variant without base type.
#[derive(Clone)]
struct EnumVariantDispatchWithTy {
	// enum ty name
	ty: Ident,
	// variant
	variant: EnumVariantDispatch,
}

impl fmt::Debug for EnumVariantDispatchWithTy {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}::{:?}", self.ty, self.variant)
	}
}

impl ToTokens for EnumVariantDispatchWithTy {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		if let Some(inner) = &self.variant.inner {
			let enum_name = &self.ty;
			let variant_name = &self.variant.name;

			let quoted = quote! {
				#enum_name::#variant_name(#inner::from(event))
			};
			quoted.to_tokens(tokens);
		}
	}
}

/// An enum variant without the base type, contains the relevant inner type.
#[derive(Clone)]
struct EnumVariantDispatch {
	/// variant name
	name: Ident,
	/// The inner type for which a `From::from` impl is anticipated from the input type.
	/// No code will be generated for this enum variant if `inner` is `None`.
	inner: Option<Type>,
}

impl fmt::Debug for EnumVariantDispatch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}(..)", self.name)
	}
}

fn prepare_enum_variant(variant: &mut Variant) -> Result<EnumVariantDispatch> {
	let skip = variant.attrs.iter().find(|attr| attr.path.is_ident("skip")).is_some();
	variant.attrs = variant.attrs.iter().filter(|attr| !attr.path.is_ident("skip")).cloned().collect::<Vec<_>>();

	let variant = variant.clone();
	let span = variant.ident.span();
	let inner = match variant.fields.clone() {
		// look for one called inner
		Fields::Named(FieldsNamed { brace_token: _, named }) if !skip => named
			.iter()
			.find_map(
				|field| {
					if let Some(ident) = &field.ident {
						if ident == "inner" {
							return Some(Some(field.ty.clone()))
						}
					}
					None
				},
			)
			.ok_or_else(|| {
				Error::new(span, "To dispatch with struct enum variant, one element must named `inner`")
			})?,

		// technically, if it has no inner types we cound not require the #[skip] annotation, but better make it consistent
		Fields::Unnamed(FieldsUnnamed { paren_token: _, unnamed }) if !skip => unnamed
			.first()
			.map(|field| Some(field.ty.clone()))
			.ok_or_else(|| Error::new(span, "Must be annotated with skip, even if no inner types exist."))?,
		_ if skip => None,
		Fields::Unit => {
			return Err(Error::new(
				span,
				"Must be annotated with #[skip].",
			))
		}
		Fields::Unnamed(_) => {
			return Err(Error::new(
				span,
				"Must be annotated with #[skip] or have in `inner` element which impls `From<_>`.",
			))
		}
		Fields::Named(_) => {
			return Err(Error::new(
				span,
				"Must be annotated with #[skip] or the first wrapped type must impl `From<_>`.",
			))
		}
	};

	Ok(EnumVariantDispatch { name: variant.ident, inner })
}

fn impl_subsystem_dispatch_gen(attr: TokenStream, item: TokenStream) -> Result<proc_macro2::TokenStream> {
	let event_ty = parse2::<Path>(attr)?;

	let mut ie = parse2::<ItemEnum>(item)?;

	let message_enum = ie.ident.clone();
	let variants = ie.variants.iter_mut().try_fold(Vec::<EnumVariantDispatchWithTy>::new(), |mut acc, variant| {
		let variant = prepare_enum_variant(variant)?;
		if variant.inner.is_some() {
			acc.push(EnumVariantDispatchWithTy { ty: message_enum.clone(), variant })
		}
		Ok::<_, syn::Error>(acc)
	})?;

	let mut orig = ie.to_token_stream();

	let msg = "Generated by #[subsystem_dispatch_gen] proc-macro.";

	orig.extend(quote! {
		impl #message_enum {
			#[doc = #msg]
			pub fn dispatch_iter(event: #event_ty) -> impl Iterator<Item=Self> + Send {
				let mut iter = None.into_iter();

				#(
					let mut iter = iter.chain(std::iter::once(event.focus().ok().map(|event| {
						#variants
					})));
				)*
				iter.filter_map(|x| x)
			}
		}
	});
	Ok(orig)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let attr = quote! {
			NetEvent<foo::Bar>
		};

		let item = quote! {
			/// Documentation.
			#[derive(Clone)]
			enum AllMessages {

				Sub1(Inner1),

				#[skip]
				/// D3
				Sub3,

				/// D4
				#[skip]
				Sub4(Inner2),

				/// D2
				Sub2(Inner2),
			}
		};

		let output = impl_subsystem_dispatch_gen(attr, item).expect("Simple example always works. qed");
		println!("//generated:");
		println!("{}", output);
	}

	#[test]
	fn ui() {
		let t = trybuild::TestCases::new();
		t.compile_fail("tests/ui/err-*.rs");
		t.pass("tests/ui/ok-*.rs");
	}
}
