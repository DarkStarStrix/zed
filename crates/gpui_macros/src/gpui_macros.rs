mod derive_into_element;
mod derive_path_static_str;
mod derive_render;
mod register_action;
mod styles;
mod test;

use proc_macro::TokenStream;

/// register_action! can be used to register an action with the GPUI runtime.
/// You should typically use `gpui::actions!` or `gpui::impl_actions!` instead,
/// but this can be used for fine grained customization.
#[proc_macro]
pub fn register_action(ident: TokenStream) -> TokenStream {
    register_action::register_action_macro(ident)
}

/// #[derive(IntoElement)] is used to create a Component out of anything that implements
/// the `RenderOnce` trait.
#[proc_macro_derive(IntoElement)]
pub fn derive_into_element(input: TokenStream) -> TokenStream {
    derive_into_element::derive_into_element(input)
}

#[proc_macro_derive(Render)]
#[doc(hidden)]
pub fn derive_render(input: TokenStream) -> TokenStream {
    derive_render::derive_render(input)
}

#[proc_macro_derive(PathStaticStr)]
#[doc(hidden)]
pub fn derive_path_static_str(input: TokenStream) -> TokenStream {
    derive_path_static_str::derive_path_static_str(input)
}

/// Used by GPUI to generate the style helpers.
#[proc_macro]
#[doc(hidden)]
pub fn style_helpers(input: TokenStream) -> TokenStream {
    styles::style_helpers(input)
}

/// Generates methods for visibility styles.
#[proc_macro]
pub fn visibility_style_methods(input: TokenStream) -> TokenStream {
    styles::visibility_style_methods(input)
}

/// Generates methods for margin styles.
#[proc_macro]
pub fn margin_style_methods(input: TokenStream) -> TokenStream {
    styles::margin_style_methods(input)
}

/// Generates methods for padding styles.
#[proc_macro]
pub fn padding_style_methods(input: TokenStream) -> TokenStream {
    styles::padding_style_methods(input)
}

/// Generates methods for position styles.
#[proc_macro]
pub fn position_style_methods(input: TokenStream) -> TokenStream {
    styles::position_style_methods(input)
}

/// Generates methods for overflow styles.
#[proc_macro]
pub fn overflow_style_methods(input: TokenStream) -> TokenStream {
    styles::overflow_style_methods(input)
}

/// Generates methods for cursor styles.
#[proc_macro]
pub fn cursor_style_methods(input: TokenStream) -> TokenStream {
    styles::cursor_style_methods(input)
}

/// Generates methods for border styles.
#[proc_macro]
pub fn border_style_methods(input: TokenStream) -> TokenStream {
    styles::border_style_methods(input)
}

/// Generates methods for box shadow styles.
#[proc_macro]
pub fn box_shadow_style_methods(input: TokenStream) -> TokenStream {
    styles::box_shadow_style_methods(input)
}

/// #[gpui::test] can be used to annotate test functions that run with GPUI support.
/// it supports both synchronous and asynchronous tests, and can provide you with
/// as many `TestAppContext` instances as you need.
/// The output contains a `#[test]` annotation so this can be used with any existing
/// test harness (`cargo test` or `cargo-nextest`).
///
/// ```
/// #[gpui::test]
/// async fn test_foo(mut cx: &TestAppContext) { }
/// ```
///
/// In addition to passing a TestAppContext, you can also ask for a `StdRnd` instance.
/// this will be seeded with the `SEED` environment variable and is used internally by
/// the ForegroundExecutor and BackgroundExecutor to run tasks deterministically in tests.
/// Using the same `StdRng` for behavior in your test will allow you to exercise a wide
/// variety of scenarios and interleavings just by changing the seed.
///
/// #[gpui::test] also takes three different arguments:
/// - `#[gpui::test(iterations=10)]` will run the test ten times with a different initial SEED.
/// - `#[gpui::test(retries=3)]` will run the test up to four times if it fails to try and make it pass.
/// - `#[gpui::test(on_failure="crate::test::report_failure")]` will call the specified function after the
///    tests fail so that you can write out more detail about the failure.
#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    test::test(args, function)
}
