use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph { 
  name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
  article: String,
  author: String,
  paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
      article: String::from("json in rust"),
      author: String::from("Mike"),
      paragraph: vec![
        Paragraph {
          name: String::from("start")
        },
        Paragraph {
          name: String::from("body")
        },
        Paragraph {
          name: String::from("end")
        }
      ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("json is {}", json)
}
