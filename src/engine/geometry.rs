pub trait Dimensions {
    fn get_rect(&self) -> (f32, f32, f32, f32);
    fn intersect(&self, other: &Dimensions) -> bool {
        let self_rect = self.get_rect();
        let other_rect = other.get_rect();
        false
        // TODO: actual check here
    }
}
