use aidoku::{
    prelude::*,
    std::{String, Vec},
    Chapter, DeepLink, Filter, Listing, Manga, MangaPageResult, Page,
};
use aidoku::error::Result;
use aidoku::std::net::{HttpMethod, Request};
use aidoku::std::html::Node;
use aidoku::std::json::JsonObject;

// Function to fetch and parse the manga list from MangaFire
#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let url = format!("https://mangafire.to/home?sort=popular&page={}", page);
    let request = Request::new(url, HttpMethod::Get);
    let html = request.html();

    let mut manga_list: Vec<Manga> = Vec::new();
    for item in html.select("div.manga-item").array() {
        let manga = parse_manga(item);
        manga_list.push(manga);
    }

    Ok(MangaPageResult {
        manga: manga_list,
        has_more: html.select("a.next-page").array().count() > 0,
    })
}

// Function to fetch and parse the manga details from MangaFire
#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
    let url = format!("https://mangafire.to/manga/{}", manga_id);
    let request = Request::new(url, HttpMethod::Get);
    let html = request.html();

    Ok(Manga {
        id: manga_id.clone(),
        title: html.select("h1.title").text().read(),
        author: html.select("div.author").text().read(),
        artist: html.select("div.artist").text().read(),
        description: html.select("div.description").text().read(),
        url: url.clone(),
        ..Default::default()
    })
}

// Function to fetch and parse the chapter list from MangaFire
#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
    let url = format!("https://mangafire.to/manga/{}/chapters", manga_id);
    let request = Request::new(url, HttpMethod::Get);
    let html = request.html();

    let mut chapters: Vec<Chapter> = Vec::new();
    for item in html.select("div.chapter-item").array() {
        let chapter = Chapter {
            id: item.select("a").attr("href").read().replace("/chapter/", ""),
            title: item.select("a").text().read(),
            chapter: item.select("span.chapter-num").text().read().parse().unwrap_or(0.0),
            url: format!("https://mangafire.to{}", item.select("a").attr("href").read()),
            ..Default::default()
        };
        chapters.push(chapter);
    }

    Ok(chapters)
}

// Function to fetch and parse the page list from MangaFire
#[get_page_list]
fn get_page_list(chapter_id: String) -> Result<Vec<Page>> {
    let url = format!("https://mangafire.to/chapter/{}", chapter_id);
    let request = Request::new(url, HttpMethod::Get);
    let html = request.html();

    let mut pages: Vec<Page> = Vec::new();
    for (index, item) in html.select("img.page-img").array().enumerate() {
        let page = Page {
            index: index as i32,
            url: item.attr("src").read(),
            ..Default::default()
        };
        pages.push(page);
    }

    Ok(pages)
}

// Function to handle deep links for MangaFire
#[handle_url]
fn handle_url(url: String) -> Result<DeepLink> {
    let url_parts: Vec<&str> = url.split('/').collect();
    if url.contains("/manga/") {
        Ok(DeepLink {
            manga: Some(get_manga_details(url_parts.last().unwrap().to_string())?),
            chapter: None,
        })
    } else if url.contains("/chapter/") {
        Ok(DeepLink {
            manga: None,
            chapter: Some(get_chapter_list(url_parts.last().unwrap().to_string())?.first().cloned().unwrap()),
        })
    } else {
        Ok(DeepLink {
            manga: None,
            chapter: None,
        })
    }
}

// Helper function to parse manga details from the HTML node
fn parse_manga(node: Node) -> Manga {
    Manga {
        id: node.select("a.manga-title").attr("href").read().replace("/manga/", ""),
        title: node.select("a.manga-title").text().read(),
        cover: node.select("img.manga-cover").attr("src").read(),
        url: format!("https://mangafire.to{}", node.select("a.manga-title").attr("href").read()),
        ..Default::default()
    }
}
