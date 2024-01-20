use leptos::*;

#[derive(Clone)]
pub struct CollectionName(pub Signal<Option<String>>);

#[derive(Clone)]
pub struct SaveAction(pub Action<(String, String, String), ()>);

impl Default for CollectionName {
    fn default() -> Self {
        Self((|| None).into())
    }
}

impl Default for SaveAction {
    fn default() -> Self {
        Self(create_action(|_| async {}))
    }
}
