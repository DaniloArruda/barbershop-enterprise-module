pub trait UseCase<P, R> {
    fn execute(&self, payload: P) -> R;
}
