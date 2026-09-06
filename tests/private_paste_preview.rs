use microbin::{private_paste_preview, Paste, PrivatePastePreview};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_a_preview_for_a_private_paste() {
        let pastes = vec![Paste {
            id: 42,
            private: true,
            content: String::from("A private draft"),
        }];

        assert_eq!(
            private_paste_preview(&pastes, 42),
            Some(PrivatePastePreview {
                id: 42,
                excerpt: String::from("A private draft"),
            })
        );
    }
}
