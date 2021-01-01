  
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct ImageUrl {
    imageUrl: String,
    text: String,
    redirectUrl: String
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct FrontCover {
    heroUrl: String,
    heading: String,
    subHeading: String;
    simpleTile: ImageUrl,
    middleTile: imageUrl,
    detailedTile: ImageUrl
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct SimpleContent {
    content: String,
    heading: String,
    subHeading: String,
    image: ImageUrl
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct MiddleContent {
    introduction: String,
    paragraphs: [String,2],
    heading: String,
    subHeading: String,
    image: ImageUrl
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct DetailedContent {
    introduction: String,
    paragraphs: [String,6],    
    heading: String,
    subHeading: String,
    dynamicContentRef: String
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
struct BlogEntry {
    id: i32,
    frontCover: FrontCover,
    section1: SimpleContent,
    section2, MiddleContent,
    section3: DetailedContent
}

impl BlogEntry {
    
}