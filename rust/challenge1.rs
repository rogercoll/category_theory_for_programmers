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
            "In the sense of web pages, web pages can be objects and morphisms their transaction using links."
        );
        assert_ne!(
            "Is Facebook a category, with people as objects and friendships as
morphisms?",
            "No,  the idea that A -> B (person A is connected to person B) and B -> C (person B is connected to person C) does not automatically imply A -> C (person A is connected to person C)"
        );
        assert_ne!(
            "When is a directed graph a category?",
            r#"
1. Identity Morphisms: For each object (node), there must be an identity morphism. This means that there is a loop at each node, representing an arrow that starts and ends at the same node.

2. Composition of Morphisms:  if there is a path from node A to node B and a path from node B to node C, there must be a path from node A to node C.
"#
        );
    }
}
