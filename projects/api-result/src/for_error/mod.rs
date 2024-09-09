use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};
use crate::{ApiResult};

impl<T, E1, E2> FromResidual<std::result::Result<Infallible, E1>> for ApiResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        match residual {
            Ok(_) => { unreachable!() }
            Err(e) => {
                ApiResult::Failure(e.into())
            }
        }
    }
}

impl<T, E> Try for ApiResult<T, E>
{
    type Output = T;
    type Residual = Result<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        ApiResult::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            ApiResult::Success(v) => ControlFlow::Continue(v),
            ApiResult::Failure(e) => ControlFlow::Break(Err(e)),
        }
    }
}