use juniper::graphql_object;
use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::models::thermostat_status::*;

pub struct ThermostatQuery;

#[graphql_object(context = Context)]
impl ThermostatQuery {
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

pub struct ThermostatMutation;

#[graphql_object(context = Context)]
impl ThermostatMutation {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(data: NewThermostatStatus) -> FieldResult<ThermostatStatus> {
        ThermostatStatus::insert(data)?;

        let result = ThermostatStatus::get_latest()?;
        Ok(result)
    }
}
