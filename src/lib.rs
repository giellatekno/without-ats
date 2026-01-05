//! # without_ats
//!
//! A one-function library with a function to remove every occurence of `"@...@"` from
//! a string.
//!
//! This is useful for us to remove the diacritics from analyser output.

/// Return an iterator of all the string slices of `s` that are not "enclosed" in "@..@".
///
/// Example:
///
/// ```
/// use without_ats::without_ats_iter;
///
/// let clean = without_ats_iter("@AAA@Clean@BBB@String@CCC@");
/// assert_eq!(String::from_iter(clean), String::from("CleanString"));
/// ```
pub fn without_ats_iter(s: &str) -> impl Iterator<Item = &str> {
    let mut i = 0;
    let mut it = memchr::memchr_iter(b'@', s.as_bytes());

    std::iter::from_fn(move || {
        if s.is_empty() {
            return None;
        }

        let lasti = s.len() - 1;

        loop {
            if i >= s.len() {
                return None;
            }

            // find opening '@'
            let Some(a) = it.next() else {
                // no next opening '@', set done and return remainder of string
                let res = Some(&s[i..]);
                i = s.len();
                return res;
            };

            // The next slice we potentially want to yield, goes from the previous `i`,
            // up to the opening '@'. Note: This may be empty, if a == i!
            let next_slice = &s[i..a];

            if a == lasti {
                // opening '@' was end of string
                i = s.len();
                return None;
            }

            // Find terminating '@'
            let Some(b) = it.next() else {
                // unterminated @, return what we have
                i = s.len();
                if !next_slice.is_empty() {
                    return Some(next_slice);
                } else {
                    return None;
                }
            };

            // now we have something to skip
            // Next, we continue from one plus b.
            i = b + 1;

            // We check if we have a slice to yield, and if not, then we just
            // continue the loop from the beginning, and look for a new slice
            if !next_slice.is_empty() {
                return Some(next_slice);
            } else {
                continue;
            }
        }
    })
}

/// Return a new `String` from the input `s`, where everything enclosed in `@...@` has
/// been removed from `s`.
/// Example:
///
/// ```
/// use without_ats::without_ats;
///
/// let clean = without_ats("@AAA@Clean@BBB@String@CCC@");
/// assert_eq!(clean, String::from("CleanString"));
/// ```
pub fn without_ats(s: &str) -> String {
    String::from_iter(without_ats_iter(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = "";
        let expected = "";
        let actual = without_ats(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn real() {
        let input = "@P.12p.add@viessat+V+IV+Imprt+Du1@R.12p.add@@D.CmpOnly.FALSE@@D.CmpPref.TRUE@@D.NeedNoun.ON@@D.SpellRlx.ON@@C.SpellRlx@@D.SpaceCmp.ON@@C.SpaceCmp@";
        let expected = "viessat+V+IV+Imprt+Du1";
        let actual = without_ats(input);
        assert_eq!(expected, actual);
    }
}
