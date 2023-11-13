#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};
/// Define a higher-order function (or a function object) memoize in
/// your favorite language. This function takes a pure function f as
/// an argument and returns a function that behaves almost exactly
/// like f, except that it only calls the original function once for every
/// argument, stores the result internally, and subsequently returns
/// this stored result every time it’s called with the same argument.
/// You can tell the memoized function from the original by watch-
/// ing its performance. For instance, try to memoize a function that
/// takes a long time to evaluate. You’ll have to wait for the result
/// the first time you call it, but on subsequent calls, with the same
/// argument, you should get the result immediately.
fn memoize<F0, F1>(f: impl Fn(F0) -> F1) -> impl FnMut(F0) -> F1
where
    F0: Eq + Hash + Copy,
    F1: Copy,
{
    let mut cached: HashMap<F0, F1> = HashMap::new();
    move |arg| {
        if !cached.contains_key(&arg) {
            cached.insert(arg, f(arg));
        }
        *cached.get(&arg).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use rand::distributions::{Distribution, Uniform};
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn memoize_works() {
        let hard_computation = Duration::from_millis(100);
        let wait_for_it = |x| {
            thread::sleep(x);
            x
        };
        let mut wait_for_it = memoize(wait_for_it);
        assert_eq!(wait_for_it(hard_computation), hard_computation);
        assert_eq!(wait_for_it(hard_computation), hard_computation);
        assert_eq!(wait_for_it(hard_computation), hard_computation);
        assert_eq!(wait_for_it(hard_computation), hard_computation);
    }

    #[test]
    /// Try to memoize a function from your standard library that you
    /// normally use to produce random numbers. Does it work?
    /// - No
    /// Most random number generators can be initialized with a seed.
    ///
    /// Implement a function that takes a seed, calls the random number
    /// generator with that seed, and returns the result. Memoize that
    /// function. Does it work?
    /// - No
    fn random_num_memoize() {
        let random = |_| rand::thread_rng().gen_range(0..1000);
        let mut num = memoize(random);
        assert!(num(()) == num(()));
        assert!(num(()) == num(()));

        let random = |seed| {
            let step = Uniform::new(0, 1000);
            step.sample(&mut StdRng::seed_from_u64(seed))
        };
        let mut num = memoize(random);

        assert!(num(222) == num(222));
        assert!(num(921) != num(222));
    }

    #[test]
    /// Try to memoize a function from your standard library that you
    /// Which of these C++ functions are pure? Try to memoize them
    /// and observe what happens when you call them multiple times:
    /// memoized and not.
    fn std_memoize_funcs() {
        assert_ne!("factorial function", "Pure function");
        assert_ne!(
            "std::get_char()",
            "No, it relies on stdin, not on the input"
        );
        assert_ne!(
            r#"
bool f() {
    std::cout << "Hello!" << std::endl;
    return true;
}
"#,
            "No, as it has side effects apart of the returned values"
        );
        assert_ne!(
            r#"
int f(int x) {
    static int y = 0;
    y += x;
    return y;
}
"#,
            "No, it relies on internal value y and it has the side effects of modifing itself"
        )
    }
}
