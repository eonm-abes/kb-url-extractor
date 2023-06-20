use std::collections::BTreeSet;
use url::Url;
/// Extrait les liens d'une page web.
pub struct LinkExtractor {
    base: Url,
}

impl LinkExtractor {
    pub fn new(base: Url) -> Self {
        Self { base }
    }

    pub fn extract<'a>(&self, html: &'a scraper::Html) -> BTreeSet<url::Url> {
        let link_selector = scraper::Selector::parse("a").expect("invalid selector");

        html.select(&link_selector)
            .map(|x| x.value().attr("href"))
            .flatten()
            .flat_map(|url| {
                Url::options()
                    .encoding_override(None)
                    .parse(&url)
                    // essaye de parser l'URL
                    .or_else(|_| {
                        // en cas d'échec (cas possible de chemin relatif) on essaye de compléter l'URL avec sa base
                        self.base.join(url)
                    })
            })
            .collect()
    }
}
