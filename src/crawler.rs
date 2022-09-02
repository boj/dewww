use std::collections::{HashSet, VecDeque};

use reqwest::Client;
use scraper::{Html, Selector};
use url::{Host, Origin, Url};

#[derive(Debug)]
pub struct Content {
    pub title: String,
    pub body: String,
    pub url: String,
}

impl Content {
    fn new() -> Self {
        Content {
            title: String::from(""),
            body: String::from(""),
            url: String::from(""),
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Crawler {
    pub root: String,
    pub local: Vec<Content>,
    pub remote: Vec<String>,
}

impl Default for Crawler {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

impl Crawler {
    pub fn new(root: String) -> Self {
        Crawler {
            root,
            local: Vec::new(),
            remote: Vec::new(),
        }
    }

    pub async fn crawl(&mut self, client: &Client, run_d: u64) -> Result<(), reqwest::Error> {
        // Use a DQ to determine which paths we still need to traverse
        let mut dq = VecDeque::new();
        dq.push_back(self.root.clone());

        // Use a Set to track which URLs we've seen
        let mut remote_set = HashSet::new();
        remote_set.insert(self.root.clone());
        let mut local_set = HashSet::new();
        local_set.insert(self.root.clone());

        while !dq.is_empty() {
            if let Some(val) = dq.pop_front() {
                println!("local: {:?}", val);

                let content = self
                    .traverse(
                        &client,
                        val.as_str(),
                        &mut dq,
                        &mut local_set,
                        &mut remote_set,
                    )
                    .await?;

                self.local.push(content);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(run_d)).await;
        }

        remote_set
            .iter()
            .for_each(|link| self.remote.push(link.clone()));

        Ok(())
    }

    async fn traverse(
        &self,
        client: &Client,
        url: &str,
        dq: &mut VecDeque<String>,
        local_set: &mut HashSet<String>,
        remote_set: &mut HashSet<String>,
    ) -> Result<Content, reqwest::Error> {
        // Track the depth we are at
        let url_parse = Url::parse(url).expect("Not a well formed URL");
        // Don't count root, thus -1
        let url_depth = url_parse.path().split("/").collect::<Vec<_>>().len() - 1;

        let res = client.get(url).send().await?;
        let bdy = res.text().await?;
        let doc = Html::parse_document(&bdy);
        let sel = Selector::parse("a").unwrap();
        let els = doc.select(&sel);

        let ttl_sel = Selector::parse("title").unwrap();
        let ttl_els = doc.select(&ttl_sel).next().unwrap();
        let ttl_txt = ttl_els.text().collect::<String>();

        // Extract the content from this url
        let mut content = Content {
            title: ttl_txt.clone(),
            url: String::from(url),
            ..Default::default()
        };
        let bdy_sel = Selector::parse("body").unwrap();
        let bdy_els = doc.select(&bdy_sel).next().unwrap();
        content.body.push_str(&bdy_els.text().collect::<String>());

        //println!("response: {:?} {}", res.version(), res.status());
        //println!("headers: {:#?}\n", res.headers());

        // loop over all of the 'a' elements and extract the links
        // as either local or remote destinations
        for el in els {
            if let Some(href) = el.value().attr("href") {
                // If it has a scheme then it's likely remote
                // otherwise a relative path local link
                match Url::parse(href) {
                    Ok(chk) => {
                        let s = chk.scheme();
                        if s == "http" || s == "https" || s == "ftp" || s == "ftps" {
                            if let Origin::Tuple(a, Host::Domain(b), _) = chk.origin() {
                                let out = [a, "://".to_string(), b].concat();
                                if !remote_set.contains(&out.clone()) {
                                    remote_set.insert(out.clone());
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Build a full path URL
                        let base = Url::parse(url).expect("local URL parse fail");
                        let link = base.join(href).expect("local URL join fail");

                        // Check the depth
                        let link_depth = link.path().split("/").collect::<Vec<_>>().len() - 1;

                        // Ignore links at the root, ourselves, and anything with a depth
                        // less than ourselves
                        if link.path() != "/" && link != url_parse && link_depth >= url_depth {
                            if !local_set.contains(&link.to_string()) {
                                local_set.insert(link.to_string());
                                dq.push_back(link.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(content)
    }
}
