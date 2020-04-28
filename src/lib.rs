extern crate iced;

mod native;
mod style;
mod wgpu;

pub use native::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
