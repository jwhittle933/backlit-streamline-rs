pub fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
    where F: Fn(A) -> B, G: Fn(B) -> C {
    move |x| g(f(x))
}
