use gstd::{Decode, Encode, String, TypeInfo};

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
    pub header: Header,
    pub content: String,
    pub footer: Footer,
}

/// Program state.
pub type State = Source;
