// You can implement index operations like foo[i] for
// your own types using the std::ops::Index, std::ops::IndeMut
// traits.

// For mutably borrowed types the expression foo[i] is
// usually a shorthand for *foo.index_mut(i), for non-mutably
// borrowed types it's instad *foo.index(i). IndexMut extends
// Index.

// The index for the index traits is a generic since you can't
// only index slices for example using `usize` but also using
// ranges of usize. Other types like HashMaps or BTreeMaps
// allow you to index using any hashable or ordered type as well.

// Here an example of an Image struct which allows you to index
// into its pixels using rows and columns.

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

use std::ops::Index;

impl<P> Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..(start + self.width)]
    }
}

use std::ops::IndexMut;

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..(start + self.width)]
    }
}

fn main() {
    let mut image = Image::new(600, 400);
    image[0][0] = 1;
    println!("image[0][0] = {}", image[0][0]);
}
