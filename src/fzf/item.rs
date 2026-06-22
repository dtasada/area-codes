use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

/// An Item in the list the user sees when searching.

#[derive(Clone)]
pub struct Item<T>
where
    T: Clone,
{
    /// This is a filler item: there isn't a search result in this place.
    pub is_blank: bool,
    pub name: String,
    pub score: Option<(i64, Vec<usize>)>,
    pub item: Option<T>,
    pub aliases: Vec<String>,
}

impl<T> Item<T>
where
    T: Clone,
{
    pub fn update_score(&mut self, search_term: &str, matcher: &SkimMatcherV2) {
        let name_score = matcher.fuzzy_indices(&self.name, search_term);
        let mut best_score = name_score;
        for alias in &self.aliases {
            if let Some(alias_score) = matcher.fuzzy_indices(alias, search_term) {
                match &best_score {
                    None => {
                        best_score = Some((alias_score.0, Vec::new()));
                    }
                    Some(current) => {
                        if alias_score.0 > current.0 {
                            best_score = Some((alias_score.0, current.1.clone()));
                        }
                    }
                }
            }
        }
        self.score = best_score;
    }

    /// Any 'new' item is always non-blank, because it has a name.
    /// Use 'empty' to create a blank item.
    pub fn new(name: String, aliases: Vec<String>, item: T) -> Self {
        Item::<T> {
            is_blank: false, // Any 'new' Item is always a non-blank.
            name,
            item: Some(item),
            score: None, // It won't be scored yet.
            aliases,
        }
    }

    /// Creates a blank item to fill in the visual space in the list.
    /// Never has an actual item attached, or a score, or a name.
    pub fn empty() -> Self {
        Item::<T> {
            is_blank: true,
            name: "".to_string(),
            score: None,
            item: None,
            aliases: Vec::new(),
        }
    }
}
