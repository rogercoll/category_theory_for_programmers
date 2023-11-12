#![allow(dead_code)]

/// Implement, as best as you can, the identity function in your favorite
/// language (or the second favorite, if your favorite language
/// happens to be Haskell).
fn id<T>(t: T) -> T {
    t
}

/// Implement the composition function in your favorite language.
/// It takes two functions as arguments and returns a function that
/// is their composition.
fn composition<T0, T1, T2>(f: impl Fn(T0) -> T1, g: impl Fn(T1) -> T2) -> impl Fn(T0) -> T2 {
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_works() {
        assert!(id(2) == 2);
        assert!(id("ale") == "ale");
        assert!(id(&"ale") == &"ale");
    }

    #[test]
    fn composition_works() {
        assert!(composition(|x| -> i8 { x + 1 }, |x| -> i8 { x - 10 })(1 as i8) == -8);
    }

    #[test]
    /// Write a program that tries to test that your composition function
    ///respects identity.
    fn composition_respects_identity() {
        assert!(composition(id, id)(2) == 2);
        assert!(composition(id, |x| -> i8 { x + 1 })(2 as i8) == 3);
        assert!(composition(|x| -> i8 { x + 1 }, id)(2 as i8) == 3)
    }

    #[test]
    ///Question and answers section
    fn question_and_answers() {
        assert_ne!(
            "Is the world-wide web a category in any sense? Are links mor-
phisms?",
            "Is the world-wide web a category in any sense? Are links mor-
phisms?"
        );
        assert_ne!(
            "Is Facebook a category, with people as objects and friendships as
morphisms?",
            "Is Facebook a category, with people as objects and friendships as
morphisms?"
        );
        assert_ne!(
            "When is a directed graph a category?",
            "When is a directed graph a category?"
        );
    }
}
