/// A single keybinding entry
#[derive(Debug, Clone)]
pub struct KeyEntry {
    pub key: String,
    pub description: String,
}

/// A named group of keybindings
#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub entries: Vec<KeyEntry>,
}

/// A top-level section (h1 heading) containing categories
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub categories: Vec<Category>,
}


/// Parse the markdown file into sections/categories/entries.
///
/// Format:
///   # Section Name        <- h1: top-level section
///   ## Category Name      <- h2: category within section
///   - `Key` — Description <- list entry
pub fn parse(text: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current_section: Option<Section> = None;
    let mut current_category: Option<Category> = None;

    let flush_category = |cat: Option<Category>, sec: &mut Option<Section>| {
        if let (Some(cat), Some(sec)) = (cat, sec.as_mut()) {
            sec.categories.push(cat);
        }
    };

    for line in text.lines() {
        let line = line.trim();

        // Skip blank lines and HTML comments
        if line.is_empty() || line.starts_with("<!--") {
            continue;
        }

        if let Some(name) = line.strip_prefix("# ") {
            // Flush previous
            flush_category(current_category.take(), &mut current_section);
            if let Some(sec) = current_section.take() {
                sections.push(sec);
            }
            current_section = Some(Section {
                name: name.trim().to_string(),
                categories: Vec::new(),
            });
        } else if let Some(name) = line.strip_prefix("## ") {
            flush_category(current_category.take(), &mut current_section);
            current_category = Some(Category {
                name: name.trim().to_string(),
                entries: Vec::new(),
            });
        } else if let Some(rest) = line.strip_prefix("- ") {
            if let Some(cat) = current_category.as_mut() {
                if let Some(entry) = parse_entry(rest) {
                    cat.entries.push(entry);
                }
            }
        }
    }

    flush_category(current_category.take(), &mut current_section);
    if let Some(sec) = current_section.take() {
        sections.push(sec);
    }

    sections
}

/// Parse a list entry like `` `Ctrl+S` — Save file ``
fn parse_entry(s: &str) -> Option<KeyEntry> {
    // Extract backtick-wrapped key
    let key_start = s.find('`')? + 1;
    let key_end = s[key_start..].find('`')? + key_start;
    let key = s[key_start..key_end].trim().to_string();

    // Everything after the closing backtick, stripping any separator chars
    // (handles ' - ', ' — ', ' – ', plain space, etc.)
    let description = s[key_end + 1..]
        .trim()
        .trim_start_matches(|c: char| !c.is_alphanumeric() && c != '(' && c != '[' && c != '<')
        .trim()
        .to_string();

    if key.is_empty() || description.is_empty() {
        return None;
    }

    Some(KeyEntry { key, description })
}
