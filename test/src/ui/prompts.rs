use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::fmt::Display;

pub fn ask_input(prompt: &str, default: Option<&str>) -> anyhow::Result<String> {
    let theme = ColorfulTheme::default();
    let input = Input::<String>::with_theme(&theme);

    // Dialoguer's builder pattern takes ownership, so we reassign
    let mut input = input.with_prompt(prompt);

    if let Some(def) = default {
        input = input.default(def.to_string());
    }

    let result = input.interact_text()?;
    Ok(result)
}

pub fn ask_select<T: Display>(
    prompt: &str,
    items: &[T],
    default_idx: usize,
) -> anyhow::Result<usize> {
    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt(prompt)
        .default(default_idx)
        .items(items)
        .interact()?;

    Ok(selection)
}

pub fn ask_multiselect<T: Display>(
    prompt: &str,
    items: &[T],
    defaults: &[bool],
) -> anyhow::Result<Vec<usize>> {
    let theme = ColorfulTheme::default();
    let selections = MultiSelect::with_theme(&theme)
        .with_prompt(prompt)
        .items(items)
        .defaults(defaults)
        .interact()?;

    Ok(selections)
}

pub fn ask_confirm(prompt: &str, default: bool) -> anyhow::Result<bool> {
    let theme = ColorfulTheme::default();
    let confirmation = Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(default)
        .interact()?;

    Ok(confirmation)
}
