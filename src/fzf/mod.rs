use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
pub use item::Item;
use list::List;
use pastel_colours::{
    BLUE_FG, DARK_BLUE_BG, DARK_GREY_BG, DARK_GREY_FG, GREEN_FG, RESET_BG, RESET_FG,
};
use std::io::{Stdout, Write, stdout};
use std::time::Instant;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};

pub mod item;
mod list;

pub struct FuzzyFinder<T>
where
    T: Clone,
{
    search_term: String,
    all_items: Vec<Item<T>>,
    matches: Vec<Item<T>>,
    stdout: RawTerminal<Stdout>,
    first: bool,
    list: List<T>,
}

impl<T> FuzzyFinder<T>
where
    T: Clone,
{
    fn new(functions: Vec<Item<T>>, lines_to_show: i8) -> Self {
        let stdout = stdout().into_raw_mode().unwrap();

        FuzzyFinder {
            search_term: String::from(""),
            all_items: functions,
            matches: vec![],
            stdout,
            first: true,
            list: List::new(lines_to_show),
        }
    }

    pub fn up(&mut self) -> Result<()> {
        self.list.up(&self.matches);
        self.update_matches();
        self.render()
    }

    pub fn down(&mut self) -> Result<()> {
        self.list.down();
        self.update_matches();
        self.render()
    }

    pub fn append(&mut self, c: char) -> Result<()> {
        self.list.reset_selection();
        // This is a normal key that we want to add to the search.
        self.search_term = format!("{}{}", self.search_term, c);

        self.update_matches();
        self.render()
    }

    pub fn backspace(&mut self) -> Result<()> {
        self.list.reset_selection();
        if self.search_term.chars().count() > 0 {
            self.search_term =
                String::from(&self.search_term[..self.search_term.chars().count() - 1]);
        }
        self.update_matches();
        self.render()
    }

    pub fn delete_word(&mut self) -> Result<()> {
        self.list.reset_selection();
        let trimmed = self.search_term.trim_end();
        if let Some(last_space_idx) = trimmed.rfind(char::is_whitespace) {
            self.search_term = trimmed[..=last_space_idx].to_string();
        } else {
            self.search_term.clear();
        }
        self.update_matches();
        self.render()
    }

    fn render_space(&mut self) -> Result<()> {
        write!(self.stdout, "{}", termion::cursor::Save).unwrap();
        if self.first {
            for _ in 0..=self.list.lines_to_show {
                writeln!(self.stdout, " ")?;
            }
            self.first = false
        }
        write!(self.stdout, "{}", termion::cursor::Restore).unwrap();

        Ok(())
    }

    fn goto_start(&mut self) -> Result<()> {
        write!(self.stdout, "\r")?;
        Ok(())
    }

    fn render_items(&mut self) -> Result<()> {
        self.goto_start()?;
        for (index, item) in self.list.items.iter().enumerate() {
            if item.is_blank {
                writeln!(self.stdout, "{}", termion::clear::CurrentLine)?;
            } else {
                let fuzzy_indecies = &item.score.as_ref().unwrap().1;

                // Do some string manipulation to colourise the indexed parts
                let coloured_line = get_coloured_line(
                    fuzzy_indecies,
                    &item.name,
                    index == self.list.selected_index as usize,
                );

                writeln!(
                    self.stdout,
                    "{}{}{}",
                    termion::clear::CurrentLine,
                    // Go maximum left, so we're at the start of the line
                    termion::cursor::Left(1000),
                    coloured_line
                )?;
            }
        }
        Ok(())
    }

    fn render_prompt(&mut self) -> Result<()> {
        write!(
            self.stdout,
            "\r{}{}{BLUE_FG}${RESET_FG} {}\r{}",
            termion::clear::CurrentLine,
            termion::cursor::Show,
            self.search_term,
            termion::cursor::Right((self.search_term.chars().count() + 2) as u16)
        )?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Gets functions that match our current criteria, sorted by score.
    pub fn update_matches(&mut self) {
        let matcher = SkimMatcherV2::default();
        for f in &mut self.all_items {
            f.update_score(&self.search_term, &matcher);
        }
        let mut matches = self
            .all_items
            .iter()
            .filter(|f| f.score.is_some())
            .cloned()
            .collect::<Vec<Item<T>>>();

        log::info!(
            "There are a total of {} item(s) and {} match(es)",
            self.all_items.len(),
            matches.len()
        );

        // We want these in the order of their fuzzy matched score, i.e. closed matches
        matches.sort_by(|a, b| b.score.cmp(&a.score));
        self.matches = matches;
        self.list.update(&self.matches);
    }

    /// Renders the current result set
    pub fn render(&mut self) -> Result<()> {
        if !self.first {
            write!(
                self.stdout,
                "{}",
                termion::cursor::Up(self.list.lines_to_show as u16)
            )?;
        }
        self.render_space()?;
        self.render_items()?;
        self.render_prompt()?;
        Ok(())
    }

    pub fn clear_all_lines(&mut self) -> Result<()> {
        let total_lines = self.list.lines_to_show as u16 + 3;
        for i in 0..total_lines {
            write!(self.stdout, "\r{}", termion::clear::CurrentLine)?;
            if i < total_lines - 1 {
                write!(self.stdout, "{}", termion::cursor::Up(1))?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }
}

/// The main entry point for the fuzzy finder.
pub fn find<T: std::clone::Clone, R: Iterator<Item = std::result::Result<Key, std::io::Error>>>(
    items: Vec<Item<T>>,
    lines_to_show: i8,
    stdin: &mut R,
) -> Result<Option<T>> {
    let mut state = FuzzyFinder::new(items, lines_to_show);

    state.update_matches();

    state.render()?;

    // Run 'sed -n l' to explore escape codes
    let mut escaped = String::from("");
    let mut instant = Instant::now();
    let mut ctrl_c_count = 0;

    loop {
        // What's going on here? The problem is how we detect escape.
        // The key presses we're interested in, e.g. the arrows, are all preceded by escape, ^[.
        // E.g. up is ^[[A and down is ^[[B. So the question is how do we identify an escape
        // key by itself? If it's ^[[A then that's ^[ followed almost instantly by [A. If we have
        // ^[ followed by a pause then we know it's not an escape for some other key, but an
        // escape by itself. That's what the 100 136His below.
        // NB: some terminals might send these bytes too slowly and escape might not be caught.
        // NB: some terminals might use different escape keys entirely.
        if escaped == "^[" && instant.elapsed().as_micros() > 100 {
            write!(state.stdout, "{}", termion::cursor::Restore)?;
            break;
        }

        if let Some(Ok(key)) = stdin.next() {
            if key != Key::Ctrl('c') {
                ctrl_c_count = 0;
            }
            match key {
                // ctrl-c and ctrl-d are two ways to exit.
                Key::Ctrl('c') => {
                    ctrl_c_count += 1;
                    if ctrl_c_count >= 2 {
                        state.clear_all_lines()?;
                        std::mem::drop(state);
                        std::process::exit(0);
                    }
                    state.search_term.clear();
                    state.update_matches();
                    state.render()?;
                }
                Key::Ctrl('d') => {
                    state.clear_all_lines()?;
                    std::mem::drop(state);
                    std::process::exit(0);
                }
                Key::Ctrl('w') => {
                    state.delete_word()?;
                }
                Key::Ctrl('p') => {
                    escaped = String::from("");
                    state.up()?;
                }
                Key::Ctrl('n') => {
                    escaped = String::from("");
                    state.down()?;
                }
                // NB: It'd be neat if we could use Key::Up and Key::Down but they don't
                // work in raw mode. So we've got to deal with the escape codes manually.

                // This captures the enter key
                Key::Char('\n') => {
                    state.clear_all_lines()?;
                    return if !state.matches.is_empty() {
                        Ok(Some(
                            state.list.get_selected().item.as_ref().unwrap().to_owned(),
                        ))
                    } else {
                        Ok(None)
                    };
                }
                Key::Char(c) => {
                    if !escaped.is_empty() {
                        escaped = format!("{}{}", escaped, c);
                        match escaped.as_str() {
                            "^[" => continue,
                            "^[[" => continue,
                            "^[[A" => {
                                escaped = String::from("");
                                state.up()?;
                            }
                            "^[[B" => {
                                escaped = String::from("");
                                state.down()?;
                            }
                            _ => {
                                // This is nothing we recognise so let's abandon the escape sequence.
                                escaped = String::from("");
                            }
                        }
                    } else {
                        state.append(c)?;
                    }
                }
                Key::Esc => {
                    // All we're doing here is recording that we've entered an escape sequence.
                    // It's actually handled when we handle chars.
                    if escaped.is_empty() {
                        escaped = String::from("^[");
                        instant = Instant::now();
                    }
                }
                Key::Backspace => {
                    state.backspace()?;
                }
                _ => {}
            }
            state.stdout.flush().unwrap();
        }
    }
    state.clear_all_lines()?;
    Ok(None)
}

/// Highlights the line. Will highlight matching search items, and also indicate
/// if it's a selected item.
fn get_coloured_line(fuzzy_indecies: &[usize], text: &str, is_selected: bool) -> String {
    // Do some string manipulation to colourise the indexed parts
    let mut coloured_line = String::from("");
    let mut start = 0;

    let text_vec = text.chars().collect::<Vec<_>>();
    for i in fuzzy_indecies {
        let part = &text_vec[start..*i].iter().cloned().collect::<String>();
        let matching_char = &text_vec[*i..*i + 1].iter().cloned().collect::<String>();
        if is_selected {
            coloured_line = format!(
                "{coloured_line}{DARK_GREY_BG}{part}{RESET_BG}{DARK_BLUE_BG}{matching_char}{RESET_BG}"
            );
        } else {
            coloured_line = format!("{coloured_line}{part}{DARK_BLUE_BG}{matching_char}{RESET_BG}");
        }
        start = i + 1;
    }
    let remaining_chars = &text_vec[start..text.chars().count()]
        .iter()
        .cloned()
        .collect::<String>();
    if is_selected {
        let prompt: String = format!("{DARK_GREY_BG}{GREEN_FG}>{RESET_FG}{RESET_BG}",);
        let spacer: String = format!("{DARK_GREY_FG}  {RESET_FG}");
        let remaining: String = format!("{DARK_GREY_BG}{remaining_chars}{RESET_BG}");
        coloured_line = format!("{prompt}{spacer}{coloured_line}{remaining}");
    } else {
        coloured_line = format!("{DARK_GREY_BG} {RESET_BG}  {coloured_line}{remaining_chars}");
    }
    coloured_line
}

#[cfg(test)]
mod tests {
    use super::Item;
    use fuzzy_matcher::skim::SkimMatcherV2;

    #[test]
    fn test_alias_matching_scores() {
        let matcher = SkimMatcherV2::default();

        // 1. Match name only
        let mut item = Item::new("Afghanistan".to_string(), vec![], ());
        item.update_score("Afghan", &matcher);
        assert!(item.score.is_some());
        let (score1, indices1) = item.score.unwrap();
        assert!(score1 > 0);
        assert!(!indices1.is_empty());

        // 2. Match alias when name doesn't match
        let mut item = Item::new(
            "Afghanistan".to_string(),
            vec!["AF".to_string(), "AFG".to_string()],
            (),
        );
        item.update_score("AF", &matcher);
        assert!(item.score.is_some());
        let (score2, _indices2) = item.score.unwrap();
        assert!(score2 > 0);
        // "AF" matches the alias "AF", name "Afghanistan" has "Af" which is case-insensitive match.
        // Wait, "Afghanistan" matches "AF" too! Let's choose an alias that is completely different.

        let mut item = Item::new(
            "Switzerland".to_string(),
            vec!["CH".to_string(), "CHE".to_string()],
            (),
        );
        item.update_score("CH", &matcher);
        assert!(item.score.is_some());
        let (score_ch, indices_ch) = item.score.unwrap();
        assert!(score_ch > 0);
        // Name "Switzerland" does not contain 'C' or 'H', so it shouldn't match at all.
        // Thus, indices should be empty.
        assert!(indices_ch.is_empty());

        // 3. Match both name and alias, verify alias score is used if higher, but name indices are kept.
        // Name: "Austria", Alias: "AUT".
        // Search term "AU" matches both.
        // "AU" in "Austria" matches indices [0, 1].
        // "AU" in "AUT" is a closer match (start of a 3-letter word vs start of a 7-letter word), so should score higher.
        let mut item = Item::new("Austria".to_string(), vec!["AUT".to_string()], ());
        item.update_score("au", &matcher);
        assert!(item.score.is_some());
        let (score_at, indices_at) = item.score.unwrap();
        assert!(score_at > 0);
        // Indices should match "Austria"'s match (i.e. [0, 1])
        assert_eq!(indices_at, vec![0, 1]);
    }
}
