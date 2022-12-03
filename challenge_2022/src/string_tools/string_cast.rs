pub trait CastTo<S> {
    fn cast_to_or_panic(&self) -> S;
    fn cast_to_opt(&self) -> Option<S>;
}

impl<S: std::str::FromStr + Default> CastTo<S> for String {

    fn cast_to_or_panic(&self) -> S {
        match self.parse::<S>() {
            Ok(value) => value,
            Err(_) => {
                S::default()
            }
        }
    }

    fn cast_to_opt(&self) -> Option<S> {
        match self.parse::<S>() {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }
}