use std::collections::BTreeSet;

/// Extrait les liens d'une page en s'appuyant sur une white list et un black list
pub struct LinkExtractor {
    base: String,
    white_list: BTreeSet<String>,
    black_list: BTreeSet<String>,
}

impl LinkExtractor {
    pub fn new(base: &str, white_list: Vec<&str>, black_list: Vec<&str>) -> Self {
        Self {
            base: base.into(),
            white_list: white_list.iter().map(ToString::to_string).collect(),
            black_list: black_list.iter().map(ToString::to_string).collect(),
        }
    }

    pub fn extract<'a>(&self, html: &'a scraper::Html) -> BTreeSet<url::Url> {
        let link_selector = scraper::Selector::parse("a").expect("invalid selector");

        let links = html
            .select(&link_selector)
            .map(|x| x.value().attr("href"))
            .flatten()
            .flat_map(|url|
                // essaye de parser l'URL
                // en cas d'échec (cas possible de chemin relatif) on essaye de compléter l'URL avec sa base
                url::Url::parse(url).or(url::Url::parse(&format!("{}{}", self.base, url))))
            .filter(|url| {
                let url = url.to_string();
                let white_list = &self.white_list;
                let black_list = &self.black_list;
                
                if white_list.is_empty() {
                    return black_list.iter().all(|b| !url.starts_with(b))
                }

                white_list.iter().any(|w| url.starts_with(w))
                    && black_list.iter().all(|b| !url.starts_with(b))
            });

        links.collect()
    }
}
