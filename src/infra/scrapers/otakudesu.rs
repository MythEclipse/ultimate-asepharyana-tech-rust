use crate::shared::utils::web::scraping::{
    attr, attr_from_or, extract_slug, selector, text, text_from_or,
};
use crate::shared::utils::parse_html;
use crate::domain::entities::anime::*;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};

/// Common selectors used across anime parsing
pub struct AnimeSelectors {
    pub item: Selector,
    pub title: Selector,
    pub link: Selector,
    pub img: Selector,
    pub episode: Selector,
    pub score: Selector,
    pub status: Selector,
    pub genre: Selector,
    pub rating: Selector,
    pub type_sel: Selector,
    pub season: Selector,
    pub desc: Selector,
}

impl AnimeSelectors {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            item: selector("article.bs").ok_or("Invalid selector: article.bs")?,
            title: selector(".tt h2").ok_or("Invalid selector: .tt h2")?,
            link: selector("a").ok_or("Invalid selector: a")?,
            img: selector("img").ok_or("Invalid selector: img")?,
            episode: selector(".epx").ok_or("Invalid selector: .epx")?,
            score: selector(".numscore").ok_or("Invalid selector: .numscore")?,
            status: selector(".status").ok_or("Invalid selector: .status")?,
            genre: selector(".genres a").ok_or("Invalid selector: .genres a")?,
            rating: selector(".score").ok_or("Invalid selector: .score")?,
            type_sel: selector(".typez").ok_or("Invalid selector: .typez")?,
            season: selector(".season").ok_or("Invalid selector: .season")?,
            desc: selector(".data .typez").ok_or("Invalid selector: .data .typez")?,
        })
    }
}

static ANIME_SELECTORS: Lazy<AnimeSelectors> = Lazy::new(|| {
    AnimeSelectors::new().expect("Valid CSS selectors")
});

pub fn extract_poster(element: &scraper::ElementRef, img_selector: &Selector) -> String {
    element
        .select(img_selector)
        .next()
        .and_then(|e| attr(&e, "src").or(attr(&e, "data-src")))
        .unwrap_or_default()
}

pub fn parse_ongoing_anime(html: &str) -> Vec<OngoingAnimeItem> {
    let document = parse_html(html);
    let selectors = &*ANIME_SELECTORS;
    let mut items = Vec::new();

    for element in document.select(&selectors.item) {
        let title = text_from_or(&element, &selectors.title, "");
        if title.is_empty() {
            continue;
        }

        let href = attr_from_or(&element, &selectors.link, "href", "");
        let slug = extract_slug(&href);
        let poster = extract_poster(&element, &selectors.img);
        let current_episode = text_from_or(&element, &selectors.episode, "N/A");
        let anime_url = attr_from_or(&element, &selectors.link, "href", "");

        items.push(OngoingAnimeItem {
            title,
            slug,
            poster,
            current_episode,
            anime_url,
        });
    }
    items
}

pub fn parse_pagination(document: &Html, current_page: u32) -> Result<Pagination, String> {
    let pagination_selector = selector(".pagination .page-numbers:not(.next)").ok_or("Invalid selector")?;
    let next_selector = selector(".pagination .next").ok_or("Invalid selector")?;

    let last_visible_page = document
        .select(&pagination_selector)
        .next_back()
        .and_then(|e| text(&e).trim().parse::<u32>().ok())
        .unwrap_or(current_page);

    let has_next_page = document.select(&next_selector).next().is_some();
    let next_page = if has_next_page { Some(current_page + 1) } else { None };
    let has_previous_page = current_page > 1;
    let previous_page = if has_previous_page { Some(current_page - 1) } else { None };

    Ok(Pagination {
        current_page,
        last_visible_page,
        has_next_page,
        next_page,
        has_previous_page,
        previous_page,
    })
}
