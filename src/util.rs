use std::borrow::Cow;

/// Provide an iterator over substrings of the given length on the given
/// source string
pub(crate) fn cut(mut source: &str, n: usize) -> impl Iterator<Item = &str> {
    std::iter::from_fn(move || {
        if source.is_empty() {
            None
        } else {
            let end_idx = source
                .char_indices()
                .nth(n - 1)
                .map_or_else(|| source.len(), |(idx, c)| idx + c.len_utf8());

            let (sub_str, rest) = source.split_at(end_idx);
            source = rest;

            Some(sub_str)
        }
    })
}

/// Put a `&str` into ASCII uppercase.
///
/// Doesn't allocate if it already is uppercase.
pub(crate) fn to_uppercase(s: &str) -> Cow<'_, str> {
    match s.as_bytes().iter().position(u8::is_ascii_lowercase) {
        Some(pos) => {
            let mut output = s.to_owned();

            // SAFETY: Index is certain to be contained
            unsafe { output.get_unchecked_mut(pos..) }.make_ascii_uppercase();

            Cow::Owned(output)
        }
        None => Cow::Borrowed(s),
    }
}

/// Splits the first `N` characters off
pub(crate) fn split_prefix<const N: usize>(s: &str) -> (&str, &str) {
    let end_idx = s
        .char_indices()
        .nth(N - 1)
        .map_or_else(|| s.len(), |(idx, c)| idx + c.len_utf8());

    s.split_at(end_idx)
}

#[cfg(test)]
mod tests {
    #[test]
    fn cut() {
        let mut iter = super::cut("hDHrdTv2n", 2);

        assert_eq!(iter.next(), Some("hD"));
        assert_eq!(iter.next(), Some("Hr"));
        assert_eq!(iter.next(), Some("dT"));
        assert_eq!(iter.next(), Some("v2"));
        assert_eq!(iter.next(), Some("n"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn to_uppercase() {
        let upper = super::to_uppercase("MANAmE JeF");
        assert_eq!(upper.as_ref(), "MANAME JEF");

        let upper = super::to_uppercase("mAn4me jäf");
        assert_eq!(upper.as_ref(), "MAN4ME JäF");
    }

    #[test]
    fn split_prefix() {
        assert_eq!(super::split_prefix::<1>("abc"), ("a", "bc"));
        assert_eq!(super::split_prefix::<4>("abc"), ("abc", ""));
    }
}
