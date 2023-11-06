use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};

use crate::models::thermostat_status::*;

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status() -> FieldResult<ThermostatStatus> {
        let result = ThermostatStatus::get_latest()?;
        Ok(result)
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history() -> FieldResult<Vec<ThermostatStatus>> {
        let results = ThermostatStatus::get_history()?;
        Ok(results)
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(data: NewThermostatStatus) -> FieldResult<ThermostatStatus> {
        ThermostatStatus::insert(data)?;

        let result = ThermostatStatus::get_latest()?;
        Ok(result)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_context() -> Context {
    Context {}
}
