pub mod exercise;
pub mod exercise_solution;
pub mod user;

pub trait Queryable {
    type Inner;

    fn parse(inner: Self::Inner) -> Self;
}
