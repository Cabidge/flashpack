use leptos::*;

#[derive(Clone)]
pub struct CollectionName(pub Signal<Option<String>>);

impl Default for CollectionName {
    fn default() -> Self {
        Self((|| None).into())
    }
}
