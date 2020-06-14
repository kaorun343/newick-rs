pub trait FromNewick: Sized {
    fn leaf(name: String) -> Self;
    fn internal(name: String, children: Vec<Self>) -> Self;

    fn update_length(self, length: Option<f64>) -> Self;
}

pub trait ToNewick {
    type Child: ToNewick;

    fn get_name(&self) -> String;
    fn get_children<'a>(&'a self) -> Vec<&'a Self::Child>;
    fn get_length(&self) -> Option<f64>;
}
