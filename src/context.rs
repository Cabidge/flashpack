use macros::define_context;

use leptos::*;

//define_context!(CollectionName: Resource<usize, Option<String>>);
define_context!(SaveAction: Action<(String, String, String), ()>);
define_context!(Header: Signal<Option<web_sys::Element>>);

mod macros {
    macro_rules! define_context {
        ($name:ident: $ty:ty) => {
            #[derive(Clone)]
            pub struct $name($ty);

            impl $name {
                pub fn provide(value: $ty) {
                    leptos::provide_context(Self(value));
                }

                pub fn use_context() -> Option<$ty> {
                    leptos::use_context().map(|Self(value)| value)
                }
            }
        };
    }

    pub(crate) use define_context;
}
