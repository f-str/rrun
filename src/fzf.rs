use fzf_wrapped::{run_with_output, Border, Fzf, Layout};

pub fn fzf_select(names: impl IntoIterator<Item = impl Into<String>>) -> String {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        //         .border_label("Favourite Colour")
        //         .color(Color::Bw)
        //         .header("Pick your favourite colour")
        .header_first(true)
        .custom_args(vec!["--height=10".to_string()])
        .build()
        .unwrap();
    run_with_output(fzf, names).unwrap()
}

pub fn fzf_select_with_input(names: impl IntoIterator<Item = impl Into<String>>) -> Option<String> {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .header_first(true)
        .custom_args(vec!["--height=10".to_string(), "--print-query".to_string()])
        .build()
        .unwrap();
    let mut name = run_with_output(fzf, names).unwrap();

    name = name.trim().to_string();

    if name.is_empty() {
        return None;
    }

    // The output is "<input>\n<selected-item>" if there is a matching item, we only want the
    // selected item in that case
    if name.contains("\n") {
        let parts: Vec<&str> = name.split("\n").collect();
        name = parts[1].to_string();
    }
    Some(name)
}
