#![no_std]
use aidoku::{
    error::Result,
    prelude::*,
    std::{String, Vec},
    Chapter, Filter, Listing, Manga, MangaPageResult, Page,
};

#[get_manga_list]
fn get_manga_list(_: Vec<Filter>, _: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_listing]
fn get_manga_listing(_: Listing, _: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_details]
fn get_manga_details(_: String) -> Result<Manga> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_: String) -> Result<Vec<Chapter>> {
    todo!()
}

#[get_page_list]
fn get_page_list(_: String, _: String) -> Result<Vec<Page>> {
    todo!()
}