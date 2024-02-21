use gstd::{collections::BTreeMap, Decode, Encode, String, TypeInfo};

/// Profile of the content
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Profile {
    pub title: String,
    pub links: BTreeMap<String, String>,
}

/// The header of the page.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Header {
    /// Title of the page.
    pub title: String,
    /// Logo of the page.
    pub logo: Option<String>,
}

/// Source of the content
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Content {
    Profile(Profile),
    Markdown(String),
}

/// Footer abstraction.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Footer {
    /// Centered information in footer.
    info: String,
}

/// Source of the page.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Source {
    header: Header,
    content: Content,
    footer: Footer,
}

/// Program state.
pub type State = BTreeMap<String, Profile>;
