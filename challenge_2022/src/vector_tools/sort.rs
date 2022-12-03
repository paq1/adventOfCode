pub trait Sort<S> {
    fn sort_immut(&self) -> Vec<S>;
}

impl<S: PartialEq + Clone + std::cmp::Ord> Sort<S> for Vec<S> {

    fn sort_immut(&self) -> Vec<S> {
        let mut res = self.clone();
        res.sort();
        res
    }
}