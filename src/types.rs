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
pub struct SiteData {
    pub root: String,
    pub local: Vec<Content>,
    pub remote: Vec<String>,
}

impl Default for SiteData {
    fn default() -> Self {
        SiteData {
            root: String::from(""),
            local: Vec::new(),
            remote: Vec::new(),
        }
    }
}

impl SiteData {
    pub fn new(root: String) -> Self {
        SiteData {
            root,
            ..Default::default()
        }
    }
}
