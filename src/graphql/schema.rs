use juniper::EmptySubscription;
use juniper::{graphql_object, RootNode};

use crate::graphql::thermostat::ThermostatMutation;
use crate::graphql::thermostat::ThermostatQuery;

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub struct RootQuery;

#[graphql_object(context = Context)]
impl RootQuery {
    fn thermostats(&self) -> ThermostatQuery {
        ThermostatQuery
    }
}

pub struct RootMutation;

#[graphql_object(context = Context)]
impl RootMutation {
    fn thermostats(&self) -> ThermostatMutation {
        ThermostatMutation
    }
}

pub type SchemaGraphQL = RootNode<'static, RootQuery, RootMutation, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(RootQuery {}, RootMutation {}, EmptySubscription::new())
}

pub fn create_context() -> Context {
    Context {}
}
