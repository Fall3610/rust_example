pub fn divide(x:f32, y:f32) -> Option<f32> {
    if y==0.0{
        None
    } else {
        Some(x/y)
    }
}
