use std::error::Error;

// use failure;
// use headless_chrome::Browser;
// use scraper;
// use std::iter::zip;
use animanga_scrapper;
use rusqlite::Connection;

// Name your user agent for browser

//Struct with base info ( title, link, current chapter)

//create the basic manga list with some entries
fn build_manga_list() -> Vec<animanga_scrapper::Manga> {
    let scrap_list = vec![
        animanga_scrapper::Manga {
            title: "Tower of God".to_string(),
            base_scrap_link: String::from("https://alpha-scans.org/tower-of-god-chapter-"),
            current_chapter: 548,
            last_chapter: 548,
            animelist_id: Some(122663),
            manga_site: animanga_scrapper::Mangasite::Alphascans,
        },
        animanga_scrapper::Manga {
            title: "Omniscient Reader’s Viewpoint Chapter".to_string(),
            base_scrap_link: String::from(
                "https://flamescans.org/1656345662-omniscient-readers-viewpoint-chapter-",
            ),
            current_chapter: 112,
            last_chapter: 112,
            animelist_id: Some(132214),
            manga_site: animanga_scrapper::Mangasite::Flamescans,
        },
        animanga_scrapper::Manga {
            title: String::from("A Returner’s Magic Should be Special"),
            base_scrap_link: String::from(
                "https://cosmicscans.com/a-returners-magic-should-be-special-chapter-",
            ),
            current_chapter: 194,
            last_chapter: 194,
            animelist_id: Some(132247),
            manga_site: animanga_scrapper::Mangasite::Cosmicscans,
        },
        animanga_scrapper::Manga {
            title: String::from("Mashle: Magic and Muscles"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100083"),
            current_chapter: 113,
            last_chapter: 113,
            animelist_id: Some(124085),
            manga_site: animanga_scrapper::Mangasite::Mangaplus,
        },
        animanga_scrapper::Manga {
            title: String::from("Boku no Hero"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100017"),
            current_chapter: 357,
            last_chapter: 357,
            animelist_id: Some(75989),
            manga_site: animanga_scrapper::Mangasite::Mangaplus,
        },
        animanga_scrapper::Manga {
            title: String::from("Jujutsu Kaisen"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100034"),
            current_chapter: 189,
            last_chapter: 189,
            animelist_id: Some(113138),
            manga_site: animanga_scrapper::Mangasite::Mangaplus,
        },
        animanga_scrapper::Manga {
            title: String::from("One Piece"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
            current_chapter: 1053,
            last_chapter: 1053,
            animelist_id: Some(13),
            manga_site: animanga_scrapper::Mangasite::Mangaplus,
        },
        animanga_scrapper::Manga {
            title: String::from("Kaiju Monster #8"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100110"),
            current_chapter: 65,
            last_chapter: 65,
            animelist_id: Some(127907),
            manga_site: animanga_scrapper::Mangasite::Mangaplus,
        },
    ];
    scrap_list
}

//scrap all elements of the list
fn scrap_all(scrap_list: &Vec<animanga_scrapper::Manga>) {
    for element in scrap_list {
        element.scrap_manga();
    }
}

// fn add_chapter_to_all(manga_list: &mut Vec<Manga>){
//     for element in manga_list {
//         element.add_chapter();
//     }
// }

fn run() {
    println!("Welcome to the manga scrapper");
    println!("--------------------------------");
    let manga_list = build_manga_list();
    //add_chapter_to_all(&mut manga_list);

    scrap_all(&manga_list);
}

fn main() {
    

}

//print variable type
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
