use crate::State;
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot;

#[derive(MergedObject, Default)]
pub struct MutationRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_schema(state: State) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
  Schema::build(
      QueryRoot::default(),
      MutationRoot::default(),
      EmptySubscription,
  )
  .data(state)
  .finish()
}
