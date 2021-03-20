use crate::entity::{
    ApiResponse, ChapterMetadata, ListQuery, NovelMeateData, ReadQuery, SearchQuery,
};
use actix_web::{get, web, HttpResponse, Responder};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[get("/search")]
pub async fn search(web::Query(query): web::Query<SearchQuery>) -> impl Responder {
    let url = format!("https://www.qidian.com/search?kw={}", query.name);

    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.bytes().await.unwrap();

    let doc = Document::from_read(body.as_ref()).unwrap();
    let mut v = vec![];

    for node in doc.find(Class("res-book-item")) {
        let mut meatedata = NovelMeateData::builder();

        node.find(Class("book-img-box").descendant(Name("img")))
            .for_each(|node| {
                meatedata.cover(node.attr("src").unwrap().to_string());
            });
        node.find(Class("book-mid-info")).for_each(|node| {
            node.find(Name("h4").descendant(Name("a")))
                .for_each(|node| {
                    meatedata
                        .name(node.text())
                        .url(node.attr("href").unwrap().to_string());
                });

            node.find(Class("author").descendant(Class("name")))
                .for_each(|node| {
                    meatedata.author(node.text());
                });

            node.find(Class("intro")).for_each(|node| {
                meatedata.intro(node.text().trim().to_string());
            });

            node.find(Class("update").descendant(Name("a")))
                .for_each(|node| {
                    meatedata.lastchapter(node.text());
                });
        });

        // node.find(Class("author").descendant(Class("name")))
        //     .for_each(|node| {})
        v.push(meatedata)
    }
    HttpResponse::Ok().json(ApiResponse::data(200, "done".to_string(), v))
}

#[get("/list")]
pub async fn list(web::Query(query): web::Query<ListQuery>) -> impl Responder {
    let url = query.url;
    let res = reqwest::get(&url).await.unwrap();

    let body = res.bytes().await.unwrap();

    let doc = Document::from_read(body.as_ref()).unwrap();

    let mut v = vec![];

    let mut serial = 1_usize;
    for node in doc.find(Attr("id", "j-catalogWrap").descendant(Class("volume-wrap"))) {
        node.find(Name("ul").descendant(Name("li")).descendant(Name("a")))
            .for_each(|node| {
                let mut meatedata = ChapterMetadata::builder();
                meatedata
                    .name(node.text())
                    .serial(serial)
                    .url(node.attr("href").unwrap().to_string());
                v.push(meatedata);
                serial = serial + 1;
            })
    }

    HttpResponse::Ok().json(ApiResponse::data(200, "done".to_string(), v))
}

#[get("read")]
pub async fn read(web::Query(query): web::Query<ReadQuery>) -> impl Responder {
    let url = query.url;

    let res = reqwest::get(&url).await.unwrap();

    let body = res.bytes().await.unwrap();

    let doc = Document::from_read(body.as_ref()).unwrap();

    let mut v = vec![];

    let node = doc.find(Class("read-content").descendant(Name("p")));

    node.for_each(|node| v.push(node.text()));

    HttpResponse::Ok().json(ApiResponse::data(200, "done".to_string(), v))
}
