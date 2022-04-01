/// Joins file paths
///
/// Intention is to perform path joining with basic two-dot normalization as fast as possible.
/// Two-dot normalization will only happen for two-dots at the beginning of rhs path.
/// Both paths expected to be in posix-style (ie. with forward slash separators).
pub fn join_paths(lhs: &str, rhs: &str) -> String {
    let mut lhs = lhs.trim_end_matches(|c| c != '/');
    let mut rhs = rhs;

    while rhs.starts_with("../") {
        lhs = lhs.trim_end_matches(|c| c != '/');
        lhs = lhs.strip_suffix('/').unwrap_or("");
        lhs = lhs.trim_end_matches(|c| c != '/');
        rhs = rhs.strip_prefix("../").unwrap_or("");
    }

    lhs.to_owned() + rhs
}

#[cfg(test)]
mod join_paths {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(join_paths("a/b/c", "d"), "a/b/d");
        assert_eq!(join_paths("a/b/", "d"), "a/b/d");
        assert_eq!(join_paths("a/b/", "c/d"), "a/b/c/d");
    }

    #[test]
    fn two_dot_normalization() {
        assert_eq!(join_paths("a/b/", "../d"), "a/d");
        assert_eq!(join_paths("a/b/c", "../d"), "a/d");
        assert_eq!(join_paths("a/b/c", "../../d"), "d");
        assert_eq!(join_paths("a/b/c", "../../../d"), "d");
    }

    #[test]
    fn exceptions() {
        // doesn't normalize one dot
        assert_eq!(join_paths("a/./b/c", "./d"), "a/./b/./d");

        // doesn't normalize two dots not at the beginning of rhs
        assert_eq!(join_paths("a/../b/", "c/../d"), "a/../b/c/../d");

        // doesn't work with back slash separators
        assert_eq!(join_paths("a\\b\\c", "d"), "d");
        assert_eq!(join_paths("a/b/c", "..\\d"), "a/b/..\\d");
    }
}
