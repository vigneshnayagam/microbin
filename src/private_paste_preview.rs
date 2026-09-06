use serde::Serialize;

#[derive(Clone)]
pub struct Paste {
    pub id: u64,
    pub private: bool,
    pub content: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct PrivatePastePreview {
    pub id: u64,
    pub excerpt: String,
}

pub fn private_paste_preview(pastes: &[Paste], id: u64) -> Option<PrivatePastePreview> {
    let paste = pastes
        .iter()
        .find(|paste| paste.id == id && paste.private)?;

    Some(PrivatePastePreview {
        id: paste.id,
        excerpt: paste.content.chars().take(140).collect(),
    })
}
