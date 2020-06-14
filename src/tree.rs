pub trait FromNewick: Sized {
    fn leaf(name: String) -> Self;
    fn internal(name: String, children: Vec<Self>) -> Self;

    fn update_length(self, length: Option<f64>) -> Self;
}
