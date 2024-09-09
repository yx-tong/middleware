use crate::PoemResult;
use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
};

impl<T, E1, E2> FromResidual<std::result::Result<Infallible, E1>> for PoemResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        match residual {
            Ok(_) => {
                unreachable!()
            }
            Err(e) => PoemResult::Failure(e.into()),
        }
    }
}

impl<T, E> Try for PoemResult<T, E> {
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        PoemResult::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            PoemResult::Success(v) => ControlFlow::Continue(v),
            PoemResult::Failure(e) => ControlFlow::Break(Err(e)),
        }
    }
}
