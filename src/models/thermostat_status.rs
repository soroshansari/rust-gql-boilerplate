use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

use diesel::prelude::*;

use crate::{db::get_connection, schema::thermostat_status};

#[derive(GraphQLObject, Queryable, Clone)]
#[graphql(description = "Thermostat status")]
pub struct ThermostatStatus {
    id: i32,
    pub status: bool,
    timestamp: NaiveDateTime,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "thermostat_status"]
#[graphql(description = "New thermostat status")]
pub struct NewThermostatStatus {
    status: bool,
}

impl ThermostatStatus {
    pub fn get_latest() -> QueryResult<ThermostatStatus> {
        let mut con = get_connection();
        use crate::schema::thermostat_status::dsl::*;

        thermostat_status
            .order(timestamp.desc())
            .limit(1)
            .get_result(&mut *con)
    }

    pub fn get_history() -> QueryResult<Vec<ThermostatStatus>> {
        let mut con = get_connection();
        use crate::schema::thermostat_status::dsl::*;

        thermostat_status
            .order(timestamp.desc())
            .limit(20)
            .load::<ThermostatStatus>(&mut *con)
    }

    pub fn insert(data: NewThermostatStatus) -> QueryResult<usize> {
        let mut con = get_connection();
        diesel::insert_into(thermostat_status::table)
            .values(&data)
            .execute(&mut *con)
    }
}
