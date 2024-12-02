pub trait MapTuple: Iterator {
    fn map_tuple<A, B, C, D, F, G>(self, f: F, g: G) -> MapTupleAdapter<Self, F, G>
    where
        Self: Iterator<Item = (A, B)> + Sized,
        F: FnMut(A) -> C,
        G: FnMut(B) -> D,
    {
        MapTupleAdapter::new(self, f, g)
    }
}

impl<I, A, B> MapTuple for I where I: Iterator<Item = (A, B)> {}

pub struct MapTupleAdapter<I, F, G> {
    iter: I,
    f: F,
    g: G,
}

impl<I, F, G, A, B, C, D> MapTupleAdapter<I, F, G>
where
    I: Iterator<Item = (A, B)>,
    F: FnMut(A) -> C,
    G: FnMut(B) -> D,
{
    const fn new(iter: I, f: F, g: G) -> Self {
        Self { iter, f, g }
    }
}

impl<I, F, G, A, B, C, D> Iterator for MapTupleAdapter<I, F, G>
where
    I: Iterator<Item = (A, B)>,
    F: FnMut(A) -> C,
    G: FnMut(B) -> D,
{
    type Item = (C, D);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(a, b)| ((self.f)(a), (self.g)(b)))
    }
}
