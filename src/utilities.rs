pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    return (y * width) + x;
}