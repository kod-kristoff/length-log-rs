pub mod app;
pub mod error;

pub use app::App;
pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
