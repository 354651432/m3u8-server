fn main() {
    let ref a = A {};
    f(a);
}

struct A;
fn f(a: &A) {}
