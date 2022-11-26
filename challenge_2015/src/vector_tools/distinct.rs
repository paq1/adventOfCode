pub trait Distinct<S> {
    fn distinct(&self) -> Vec<S>;
}

impl<S: PartialEq + Clone> Distinct<S> for Vec<S> {

    fn distinct(&self) -> Vec<S> {
        self
            .iter()
            .fold(vec![], |acc, item| {
                if !acc.contains(item) {
                    [acc, vec![item.clone()]].concat()
                } else {
                    acc
                }
            })
    }
}