pub mod core;

fn main() {
    let mut input: [u32; 6] = [2, 3, 6, 5, 1, 100];
    core::bubble_sort(&mut input, core::SortMethod::Desc);
}
