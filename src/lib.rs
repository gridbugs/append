use std::collections::VecDeque;

pub trait Append<T> {
    type Common: From<T>;
    fn append(&mut self, value: T);
}

impl<T, S: From<T>> Append<T> for Vec<S> {
    type Common = S;
    fn append(&mut self, value: T) {
        self.push(value.into());
    }
}

impl<T, S: From<T>> Append<T> for VecDeque<S> {
    type Common = S;
    fn append(&mut self, value: T) {
        self.push_back(value.into());
    }
}

#[test]
fn foo() {
    enum Foo {
        Bar,
        Baz(u64),
    }

    impl From<u64> for Foo {
        fn from(u: u64) -> Self {
            Foo::Baz(u)
        }
    }

    fn f<A: Append<Foo> + Append<u64>>(a: &mut A) {
        a.append(Foo::Bar);
        a.append(0);
    }

    let mut v: Vec<Foo> = Vec::new();

    f(&mut v);
}
