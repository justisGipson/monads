trait Pointed where Self: Sized {
    type Unit;

    fn of(unit: Self::Unit) -> Self;

    fn unwrap(self) -> Self::Unit;
}

trait Functor: Pointed {
    fn map<B, F>(self, f: F) -> B
        where B: Functor,
              F: Fn(Self::Unit) -> B::Unit {
                  B::of(f(self.unwrap()))
              }
}

trait Monad: Functor {
    fn chain<M, F>(self, f: F) -> M
        where M: Monad,
              F: Fn(Self::Unit) -> M {
                  f(self.unwrap())
              }
}

#[derive(Copy, Clone, Debug)]
struct Identity<T>(T);

impl<T> Pointed for Identity<T> {
    type Unit = T;
    fn of(unit: Self::Unit) -> Self {
        Identity(unit)
    }
    fn unwrap(self) -> Self::Unit {
        self.0
    }
}

impl<T> Functor for Identity<T> {}
impl<T> Monad for Identity<T> {}

#[derive(Copy, Clone, Debug)]
struct Pair<A, B>(A, B);

impl<A, B> Pointed for Pair<A, B> {
    type Unit = (A, B);
    fn of(unit: Self::Unit) -> Self {
        Pair(unit.0, unit.1)
    }
    fn unwrap(self) -> Self::Unit {
        (self.0, self.1)
    }
}

impl<A, B> Functor for Pair<A, B> {}
impl<A, B> Monad for Pair<A, B> {}

fn main() {
    let id1 = Identity::of(5);
    let id2: Identity<i32> = id1.map(|x| x * 3);
    let id3 = id2.chain(|x| Identity::of(x - 3));

    println!("{:?}", id1);
    println!("{:?}", id2);
    println!("{:?}", id3);

    let p1 = Pair::of((1, 2));
    let p2: Pair<u32, u32> = p1.map(|(a, b)| (a * 2, b + 3));
    let p3 = p2.chain(|(a, b)| Pair::of((b, a)));

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
}
