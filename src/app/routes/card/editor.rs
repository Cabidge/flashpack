use leptos::*;

#[derive(Clone, Copy)]
struct RwSection {
    id: u64,
    value: RwSignal<String>,
}

#[derive(Clone)]
struct RwSections {
    uid: u64,
    sections: Vec<RwSection>,
}

impl RwSection {
    fn dispose(self) {
        self.value.dispose();
    }
}

impl RwSections {
    fn new(initial_contents: &str) -> Self {
        let mut sections = RwSections {
            uid: 0,
            sections: vec![],
        };

        for section in crate::slides::ThematicBreaks::new(initial_contents) {
            let initial_value = section.trim().to_string();
            sections.push(initial_value);
        }

        if sections.sections.is_empty() {
            sections.push(String::new());
        }

        sections
    }

    /// Get the next unique id.
    fn next_id(&mut self) -> u64 {
        let id = self.uid;
        self.uid += 1;
        id
    }

    /// Create a new section with a unique id.
    fn create_section(&mut self, initial_value: String) -> RwSection {
        let id = self.next_id();
        let value = create_rw_signal(initial_value);
        RwSection { id, value }
    }

    fn get(&self, id: u64) -> Option<(usize, RwSection)> {
        self.sections
            .iter()
            .copied()
            .enumerate()
            .find(|(_, section)| section.id == id)
    }

    /// Add a section directly after a specific section.
    /// Return the id of the new section.
    fn insert_after(&mut self, id: u64, initial_value: String) -> u64 {
        let Some((index, _)) = self.get(id) else {
            logging::warn!("Section editor with id of {} not found", id);
            return id;
        };

        let section = self.create_section(initial_value);
        self.sections.insert(index + 1, section);

        section.id
    }

    /// Add a section to the end.
    /// Returns the id of the new section.
    fn push(&mut self, initial_value: String) -> u64 {
        let section = self.create_section(initial_value);
        self.sections.push(section);

        section.id
    }

    /// Removes and disposes of a RwSection.
    /// Returns the id of the section that preceeds it.
    fn remove(&mut self, id: u64) -> u64 {
        // there needs to be at least one section
        if self.sections.len() <= 1 {
            return id;
        }

        let Some((index, _)) = self.get(id) else {
            logging::warn!("Section editor with id of {} not found", id);
            return id;
        };

        let section = self.sections.remove(index);
        section.dispose();

        let preceeding = index.saturating_sub(1);
        self.sections[preceeding].id
    }

    fn split(&mut self, id: u64) -> u64 {
        let Some((index, section)) = self.get(id) else {
            logging::warn!("Section editor with id of {} not found", id);
            return id;
        };

        let content = section.value.get_untracked();
        let mut breaks = crate::slides::ThematicBreaks::new(&content);

        let Some(content) = breaks.next() else {
            return id;
        };

        // truncate the first section
        section
            .value
            .update(|value| value.truncate(content.trim_end().len()));

        let mut last_id = id;
        // this is O(n^2), but under normal circumstances
        // this loop will only go through one iteration anyways
        for (offset, section) in breaks.enumerate() {
            let section = self.create_section(section.trim().to_string());
            let index = index + 1 + offset;
            self.sections.insert(index, section);

            last_id = section.id;
        }

        last_id
    }
}

fn can_split(s: &str) -> bool {
    crate::slides::ThematicBreaks::new(s).nth(1).is_some()
}

#[component]
pub fn SectionsEditor(
    initial_contents: String,
    #[prop(into)] on_discard: Callback<()>,
    #[prop(into)] on_save: Callback<String>,
) -> impl IntoView {
    let sections = create_rw_signal(RwSections::new(&initial_contents));

    let (force_focus, set_force_focus) = create_signal(0);

    let add_to_end = move || {
        if let Some(id) = sections.try_update(|sections| sections.push(String::new())) {
            set_force_focus.set(id);
        }
    };

    let add_after = move |id| {
        if let Some(id) = sections.try_update(|sections| sections.insert_after(id, String::new())) {
            set_force_focus.set(id);
        }
    };

    let delete = move |id| {
        if let Some(id) = sections.try_update(|sections| sections.remove(id)) {
            set_force_focus.set(id);
        }
    };

    let split = move |id| {
        if let Some(id) = sections.try_update(|sections| sections.split(id)) {
            set_force_focus.set(id);
        }
    };

    let on_save_click = move |_| {
        let formatted = sections.with(|sections| {
            let mut formatted = String::new();
            let mut is_first = true;
            for section in sections.sections.iter() {
                if !is_first {
                    formatted.push_str("\n\n---\n\n");
                }
                is_first = false;

                section.value.with(|section| {
                    formatted.push_str(section);
                });
            }

            formatted
        });

        on_save.call(formatted);
    };

    let is_focused = move |id| force_focus.get() == id;
    let create_focused = move |id| create_memo(move |_| is_focused(id));

    view! {
        <div class="section-editor">
            <For
                each=move || sections.get().sections
                key=|section| section.id
                let:section
            >
                <Editor
                    contents={section.value}
                    is_force_focused=create_focused(section.id)
                    on_add=move |_| add_after(section.id)
                    on_delete=move |_| delete(section.id)
                    on_split=move |_| split(section.id)
                />
            </For>
            <button on:click=move |_| add_to_end()>
                "+"
            </button>
        </div>
        <div class="editor-button-group">
            <button on:click=move |_| on_discard.call(())>
                "Cancel"
            </button>
            <button class="primary" on:click=on_save_click>
                "Save"
            </button>
        </div>
    }
}

#[component]
fn Editor(
    contents: RwSignal<String>,
    is_force_focused: Memo<bool>,
    #[prop(into)] on_add: Callback<()>,
    #[prop(into)] on_delete: Callback<()>,
    #[prop(into)] on_split: Callback<()>,
) -> impl IntoView {
    let on_input = move |ev| {
        let new_value = event_target_value(&ev);

        // prevent double triggering the autosize
        batch(move || {
            contents.set(new_value);
            if contents.with(|content| can_split(content)) {
                on_split.call(());
            }
        });
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        let code = ev.code();

        logging::warn!("{code}");

        // if backspace is pressed on an already empty editor
        if contents.with(String::is_empty) && code == "Backspace" {
            ev.prevent_default();
            on_delete.call(());
        } else if code == "KeyN" && ev.ctrl_key() {
            on_add.call(());
        }
    };

    let node_ref = create_node_ref::<html::Textarea>();

    // autosize
    create_effect(move |_| {
        contents.track();

        if let Some(node_ref) = node_ref.get() {
            let node_ref = node_ref.style("height", "auto");
            let new_height = format!("{}px", node_ref.scroll_height());
            let _ = node_ref.style("height", new_height);
        }
    });

    // force focus
    create_effect(move |_| {
        if is_force_focused.get() {
            if let Some(node_ref) = node_ref.get() {
                let _ = node_ref.focus().ok();
            }
        }
    });

    view! {
        <div class="editor">
            <textarea
                node_ref=node_ref
                prop:value=move || contents.get()
                on:input=on_input
                on:keydown=handle_keydown
                rows="1"
            >
                {contents.get_untracked()}
            </textarea>
        </div>
    }
}
