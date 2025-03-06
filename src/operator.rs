use async_trait::async_trait;

use crate::Ham;
use crate::State;

#[async_trait]
pub trait Operator {
    async fn ham(&self) -> Ham;
    async fn state(&self) -> State;
}
