use std::fmt::Debug;

use url::Url;

pub trait Control<S_ITEM, C_ITEM> {
    fn control(input: &S_ITEM, list: &[C_ITEM]) -> bool;
}

#[derive(Debug, Clone)]
/// Contient une liste d'éléments à vérifier
pub struct ControlList<C_ITEM: Clone + Debug>(Vec<C_ITEM>);

impl<C_ITEM: Clone + Debug> Default for ControlList<C_ITEM> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<C_ITEM: Ord + Clone + Debug> ControlList<C_ITEM> {
    pub fn new(input: Vec<C_ITEM>) -> Self {
        let mut input = input;
        input.sort();

        ControlList(input)
    }

    pub fn includes<S_ITEM, C: Control<S_ITEM, C_ITEM>>(&self, input: &S_ITEM) -> bool {
        if !self.0.is_empty() {
            C::control(input, &self.0[..])
        } else {
            true
        }
    }
}

/// Vérifie que les chemins des deux URL sont identiques
pub struct MatchUrlPath;

impl Control<Url, Url> for MatchUrlPath {
    fn control(input: &Url, list: &[Url]) -> bool {
        let input_path = input.path();
        list.iter()
            .map(|url| url.path())
            .any(|path| path == input_path)
    }
}

/// Vérifie que le début des chemins des deux URL sont identiques
pub struct MatchStartUrlPath;

impl Control<Url, Url> for MatchStartUrlPath {
    fn control(input: &Url, list: &[Url]) -> bool {
        let input_path = input.path();
        list.iter()
            .map(|url| url.path())
            .any(|path| input_path.starts_with(path))
    }
}

/// Vérifie que la fin des chemins des deux URL sont identiques
pub struct MatchEndUrlPath;

impl Control<Url, String> for MatchEndUrlPath {
    fn control(input: &Url, list: &[String]) -> bool {
        list.iter().any(|elem| input.path().ends_with(elem))
    }
}

pub struct LinkFilter<C_ITEM: Clone + Debug + Ord> {
    white_list: ControlList<C_ITEM>,
    black_list: ControlList<C_ITEM>,
}

impl<C_ITEM: Clone + Debug + Ord> LinkFilter<C_ITEM> {
    pub fn new(white_list: ControlList<C_ITEM>, black_list: ControlList<C_ITEM>) -> Self {
        LinkFilter {
            white_list,
            black_list,
        }
    }

    pub fn filter<X, C: Control<X, C_ITEM>>(self, links: &mut Vec<X>) {
        links
            // .iter()
            .drain_filter(|link| !{
                // .filter(|link| {
                !self.black_list.includes::<X, C>(link) || self.white_list.includes::<X, C>(link)
            });
        // .collect()
    }
}
