use gstd::{collections::BTreeMap, Decode, Encode, String, TypeInfo, Vec};

/// Profile of the content
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Profile {
    pub title: String,
    pub links: BTreeMap<String, String>,
}

/// The header of the page.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Header {
    /// Title of the page.
    pub title: String,
    /// Logo of the page.
    pub logo: Option<String>,
}

/// Source of the content
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Content {
    Profile(Profile),
    Markdown(String),
}

impl Content {
    /// If the content contains the token.
    pub fn contains(&self, token: &str) -> bool {
        match self {
            Content::Profile(profile) => profile.title.contains(token),
            Content::Markdown(markdown) => markdown.contains(token),
        }
    }
}

/// Footer abstraction.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Footer {
    /// Centered information in footer.
    pub info: String,
}

/// Source of the page.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Source {
    /// Labels for querying the pages.
    pub labels: Vec<String>,
    pub header: Header,
    pub content: Content,
    pub footer: Footer,
}

/// Program state.
pub type State = BTreeMap<String, Source>;
