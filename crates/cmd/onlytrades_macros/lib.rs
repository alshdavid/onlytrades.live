use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &input_fn.sig.ident,
            "#[onlytrades::main] can only be applied to `async` functions",
        )
        .to_compile_error()
        .into();
    }

    // 1. Extract BOTH the name and the type of the context parameter (and clone them)
    let (ctx_var, ctx_type) = match input_fn.sig.inputs.first() {
        Some(FnArg::Typed(pat_type)) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => {
                let var_name = pat_ident.ident.clone();
                let var_type = (*pat_type.ty).clone(); // <--- CRITICAL FIX: Clone the type here
                (var_name, var_type)
            }
            _ => {
                return syn::Error::new_spanned(
                    &pat_type.pat,
                    "Expected a simple variable name for the context argument",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input_fn.sig.ident,
                "#[onlytrades::main] function must take a Context argument",
            )
            .to_compile_error()
            .into()
        }
    };

    // 2. Erase the async keyword
    input_fn.sig.asyncness = None;
    
    // 3. This now works perfectly because the borrow from step 1 has ended!
    input_fn.sig.inputs = Punctuated::new();

    let body = &input_fn.block;
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;

    // 4. Reconstruct the function
    let expanded = quote! {
        #(#attrs)* #vis #sig {
            onlytrades_bot::bootstrap(|#ctx_var: #ctx_type| async move #body)
        }
    };

    TokenStream::from(expanded)
}