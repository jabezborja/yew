#![allow(clippy::needless_doctest_main)]
#![doc(html_logo_url = "https://yew.rs/img/logo.png")]
#![cfg_attr(documenting, feature(doc_cfg))]

//! # Yew Framework - API Documentation
//!
//! Yew is a modern Rust framework for creating multi-threaded front-end web apps using WebAssembly
//!
//! - Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.
//! - Achieves high performance by minimizing DOM API calls for each page render and by making it easy to offload processing to background web workers.
//! - Supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.
//!
//! ### Supported Targets (Client-Side Rendering)
//! - `wasm32-unknown-unknown`
//!
//! ### Note
//!
//! Server-Side Rendering should work on all targets when feature `ssr` is enabled.
//!
//! ### Supported Features:
//! - `ssr`: Enables Server-side Rendering support and [`ServerRenderer`].
//! - `tokio`: Enables future-based APIs on non-wasm32 targets with tokio runtime. (You may want to
//! enable this if your application uses future-based APIs and it does not compile / lint on
//! non-wasm32 targets.)
//!
//! ## Example
//!
//! ```rust
//! use yew::prelude::*;
//!
//! enum Msg {
//!     AddOne,
//! }
//!
//! struct App {
//!     value: i64,
//! }
//!
//! impl Component for App {
//!     type Message = Msg;
//!     type Properties = ();
//!
//!     fn create(ctx: &Context<Self>) -> Self {
//!         Self {
//!             value: 0,
//!         }
//!     }
//!
//!     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//!         match msg {
//!             Msg::AddOne => {
//!                 self.value += 1;
//!                 true
//!             }
//!         }
//!     }
//!
//!     fn view(&self, ctx: &Context<Self>) -> Html {
//!         html! {
//!             <div>
//!                 <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!
//!# fn dont_execute() {
//! fn main() {
//!     yew::start_app::<App>();
//! }
//!# }
//! ```
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![recursion_limit = "512"]
extern crate self as yew;

use std::{cell::Cell, panic::PanicInfo};

/// This macro provides a convenient way to create [`Classes`].
///
/// The macro takes a list of items similar to the [`vec!`] macro and returns a [`Classes`] instance.
/// Each item can be of any type that implements `Into<Classes>` (See the implementations on [`Classes`] to learn what types can be used).
///
/// # Example
///
/// ```
/// # use yew::prelude::*;
/// # fn test() {
/// let conditional_class = Some("my-other-class");
/// let vec_of_classes = vec!["one-bean", "two-beans", "three-beans", "a-very-small-casserole"];
///
/// html! {
///     <div class={classes!("my-container-class", conditional_class, vec_of_classes)}>
///         // ...
///     </div>
/// };
/// # }
/// ```
pub use yew_macro::classes;

/// This macro implements JSX-like templates.
///
/// This macro always returns [`Html`].
/// If you need to preserve the type of a component, use the [`html_nested!`] macro instead.
///
/// More information about using the `html!` macro can be found in the [Yew Docs]
///
/// [`Html`]: ./html/type.Html.html
/// [`html_nested!`]: ./macro.html_nested.html
/// [Yew Docs]: https://yew.rs/docs/next/concepts/html
pub use yew_macro::html;

/// This macro is similar to [`html!`], but preserves the component type instead
/// of wrapping it in [`Html`].
///
/// That macro is useful when, for example, in a typical implementation of a list
/// component (let's assume it's called `List`).
/// In a typical implementation you might find two component types -- `List` and `ListItem`.
/// Only `ListItem` components are allowed to be children of List`.
///
/// You can find an example implementation of this in the [`nested_list`] example.
/// That example shows, how to create static lists with their children.
///
/// ```
/// # use yew::prelude::*;
/// use yew::html::ChildrenRenderer;
/// use yew::virtual_dom::VChild;
///
/// #[derive(Clone, Properties, PartialEq)]
/// struct ListProps {
///   children: ChildrenRenderer<ListItem>,
/// }
///
/// struct List;
/// impl Component for List {
/// #   type Message = ();
///   type Properties = ListProps;
///   // ...
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// }
///
/// #[derive(Clone, PartialEq)]
/// struct ListItem;
/// impl Component for ListItem {
/// #   type Message = ();
/// #   type Properties = ();
///   // ...
/// #   fn create(ctx: &Context<Self>) -> Self { Self }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// }
///
/// // Required for ChildrenRenderer
/// impl From<VChild<ListItem>> for ListItem {
///   fn from(child: VChild<ListItem>) -> Self { Self }
/// }
///
/// impl Into<Html> for ListItem {
///   fn into(self) -> Html { html! { <self /> } }
/// }
/// // You can use `List` with nested `ListItem` components.
/// // Using any other kind of element would result in a compile error.
/// # fn test() -> Html {
/// html! {
///   <List>
///     <ListItem/>
///     <ListItem/>
///     <ListItem/>
///   </List>
/// }
/// # }
/// # fn test_iter() -> Html {
/// # let some_iter = (0..10);
/// // In many cases you might want to create the content dynamically.
/// // To do this, you can use the following code:
/// html! {
///   <List>
///     { for some_iter.map(|_| html_nested!{ <ListItem/> }) }
///   </List>
/// }
/// # }
/// ```
///
/// If you used the [`html!`] macro instead of `html_nested!`, the code would
/// not compile because we explicitly indicated to the compiler that `List`
/// can only contain elements of type `ListItem` using [`ChildrenRenderer<ListItem>`],
/// while [`html!`] creates items of type [`Html`].
///
///
/// [`html!`]: ./macro.html.html
/// [`Html`]: ./html/type.Html.html
/// [`nested_list`]: https://github.com/yewstack/yew/tree/master/examples/nested_list
/// [`ChildrenRenderer<ListItem>`]: ./html/struct.ChildrenRenderer.html
pub use yew_macro::html_nested;

/// Build [`Properties`] outside of the [`html!`] macro.
///
/// It's already possible to create properties like normal Rust structs
/// but if there are lots of optional props the end result is often needlessly verbose.
/// This macro allows you to build properties the same way the [`html!`] macro does.
///
/// The macro doesn't support special props like `ref` and `key`, they need to be set in the [`html!`] macro.
///
/// You can read more about `Properties` in the [Yew Docs].
///
/// # Example
///
/// ```
/// # use yew::prelude::*;
/// use std::borrow::Cow;
///
/// #[derive(Clone, Properties, PartialEq)]
/// struct Props {
///     #[prop_or_default]
///     id: usize,
///     name: Cow<'static, str>,
/// }
///
/// struct MyComponent(Props);
/// impl Component for MyComponent {
/// #   type Message = ();
///     type Properties = Props;
///     // ...
/// #   fn create(ctx: &Context<Self>) -> Self { unimplemented!() }
/// #   fn view(&self, ctx: &Context<Self>) -> Html { unimplemented!() }
/// }
///
/// # fn foo() -> Html {
/// // You can build props directly ...
/// let props = yew::props!(Props { name: Cow::from("Minka") });
/// # assert_eq!(props.name, "Minka");
/// // ... or build the associated properties of a component
/// let props = yew::props!(MyComponent::Properties { id: 2, name: Cow::from("Lemmy") });
/// # assert_eq!(props.id, 2);
///
/// // Use the Rust-like struct update syntax to create a component with the props.
/// html! {
///     <MyComponent key=1 ..props />
/// }
/// # }
/// ```
///
/// [`html!`]: ./macro.html.html
/// [`Properties`]: ./html/trait.Properties.html
/// [Yew Docs]: https://yew.rs/concepts/components/properties
pub use yew_macro::props;

/// This module contains macros which implements html! macro and JSX-like templates
pub mod macros {
    pub use crate::classes;
    pub use crate::html;
    pub use crate::html_nested;
    pub use crate::props;
}

mod app_handle;
pub mod callback;
pub mod context;
pub mod functional;
pub mod html;
mod io_coop;
pub mod scheduler;
mod sealed;
#[cfg(feature = "ssr")]
mod server_renderer;
pub mod suspense;
#[cfg(test)]
pub mod tests;
pub mod utils;
pub mod virtual_dom;
#[cfg(feature = "ssr")]
pub use server_renderer::*;

/// The module that contains all events available in the framework.
pub mod events {
    pub use crate::html::TargetCast;

    pub use crate::virtual_dom::listeners::set_event_bubbling;

    #[doc(no_inline)]
    pub use web_sys::{
        AnimationEvent, DragEvent, ErrorEvent, Event, FocusEvent, InputEvent, KeyboardEvent,
        MouseEvent, PointerEvent, ProgressEvent, TouchEvent, TransitionEvent, UiEvent, WheelEvent,
    };
}

pub use crate::app_handle::AppHandle;
use web_sys::Element;

use crate::html::BaseComponent;

thread_local! {
    static PANIC_HOOK_IS_SET: Cell<bool> = Cell::new(false);
}

/// Set a custom panic hook.
/// Unless a panic hook is set through this function, Yew will
/// overwrite any existing panic hook when one of the `start_app*` functions are called.
pub fn set_custom_panic_hook(hook: Box<dyn Fn(&PanicInfo<'_>) + Sync + Send + 'static>) {
    std::panic::set_hook(hook);
    PANIC_HOOK_IS_SET.with(|hook_is_set| hook_is_set.set(true));
}

fn set_default_panic_hook() {
    if !PANIC_HOOK_IS_SET.with(|hook_is_set| hook_is_set.replace(true)) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}

/// The main entry point of a Yew application.
/// If you would like to pass props, use the `start_app_with_props_in_element` method.
pub fn start_app_in_element<COMP>(element: Element) -> AppHandle<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    start_app_with_props_in_element(element, COMP::Properties::default())
}

/// Starts an yew app mounted to the body of the document.
/// Alias to start_app_in_element(Body)
pub fn start_app<COMP>() -> AppHandle<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    start_app_with_props(COMP::Properties::default())
}

/// The main entry point of a Yew application. This function does the
/// same as `start_app_in_element(...)` but allows to start an Yew application with properties.
pub fn start_app_with_props_in_element<COMP>(
    element: Element,
    props: COMP::Properties,
) -> AppHandle<COMP>
where
    COMP: BaseComponent,
{
    set_default_panic_hook();
    AppHandle::<COMP>::mount_with_props(element, Rc::new(props))
}

/// The main entry point of a Yew application.
/// This function does the same as `start_app(...)` but allows to start an Yew application with properties.
pub fn start_app_with_props<COMP>(props: COMP::Properties) -> AppHandle<COMP>
where
    COMP: BaseComponent,
{
    start_app_with_props_in_element(
        gloo_utils::document()
            .body()
            .expect("no body node found")
            .into(),
        props,
    )
}

/// The Yew Prelude
///
/// The purpose of this module is to alleviate imports of many common types:
///
/// ```
/// # #![allow(unused_imports)]
/// use yew::prelude::*;
/// ```
pub mod prelude {
    pub use crate::app_handle::AppHandle;
    pub use crate::callback::Callback;
    pub use crate::context::{ContextHandle, ContextProvider};
    pub use crate::events::*;
    pub use crate::html::{
        create_portal, BaseComponent, Children, ChildrenWithProps, Classes, Component, Context,
        Html, HtmlResult, NodeRef, Properties,
    };
    pub use crate::macros::{classes, html, html_nested};
    pub use crate::suspense::Suspense;

    pub use crate::functional::*;
}

pub use self::prelude::*;
use std::rc::Rc;
