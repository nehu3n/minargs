mod app;
mod arg;
mod matches;

pub use app::App;
pub trait FromArgs: Sized {
    fn from_args() -> Self;
}
