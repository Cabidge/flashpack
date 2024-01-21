use leptos::*;

#[derive(Clone)]
pub struct CollectionName(pub Resource<usize, Option<String>>);

#[derive(Clone)]
pub struct SaveAction(pub Action<(String, String, String), ()>);

impl Default for CollectionName {
    fn default() -> Self {
        Self(create_resource(|| 0, |_| std::future::pending()))
    }
}

impl Default for SaveAction {
    fn default() -> Self {
        Self(create_action(|_| async {}))
    }
}
