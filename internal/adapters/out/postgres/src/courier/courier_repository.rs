use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use std::collections::HashMap;
use uuid::Uuid;

use crate::courier::courier_mapper::CourierRecord;
use crate::courier::courier_schema::couriers::dsl::id;
use crate::courier::courier_schema::couriers::dsl::*;
use crate::errors::postgres_error::PostgresError;
use crate::storage_place::storage_place_dto::StoragePlaceDto;
use crate::storage_place::storage_place_schema::storage_places::dsl::*;
use crate::storage_place::storage_place_schema::storage_places::order_id;

use super::courier_dto::CourierDto;

pub struct CourierRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> CourierRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        Self { connection }
    }
}

impl<'a> CourierRepositoryPort for CourierRepository<'a> {
    fn add(&mut self, c: Courier) -> Result<(), RepositoryError> {
        let dto: CourierDto = c.into();
        insert_into(couriers)
            .values(&dto)
            .execute(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn update(&mut self, c: Courier) -> Result<(), RepositoryError> {
        let dto: CourierDto = c.into();

        update(couriers.find(dto.id))
            .set(&dto)
            .execute(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn get_by_id(&mut self, c_id: CourierId) -> Result<Courier, RepositoryError> {
        let results: Vec<(CourierDto, StoragePlaceDto)> = couriers
            .inner_join(storage_places)
            .filter(id.eq(c_id.0))
            .load(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let (courier_dto, storage_dtos): (CourierDto, Vec<StoragePlaceDto>) = {
            let mut iter = results.into_iter();
            let first = iter
                .next()
                .ok_or_else(|| RepositoryError::NotFound(format!("courier {}", c_id.0)))?;

            let courier = first.0.clone();
            let mut storage = vec![first.1];
            for (_, sp) in iter {
                storage.push(sp);
            }
            (courier, storage)
        };

        let record = CourierRecord(courier_dto, storage_dtos);

        record.try_into().map_err(RepositoryError::from)
    }

    fn get_all_free(&mut self) -> Result<Vec<Courier>, RepositoryError> {
        let rows: Vec<(CourierDto, StoragePlaceDto)> = couriers
            .inner_join(storage_places)
            .filter(order_id.is_null())
            .load(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let mut grouped: HashMap<Uuid, (CourierDto, Vec<StoragePlaceDto>)> = HashMap::new();

        for (c_dto, sp_dto) in rows {
            grouped
                .entry(c_dto.id)
                .or_insert_with(|| (c_dto.clone(), Vec::new()))
                .1
                .push(sp_dto);
        }

        grouped
            .into_values()
            .map(|(c_dto, sp_dtos)| {
                CourierRecord(c_dto, sp_dtos)
                    .try_into()
                    .map_err(RepositoryError::from)
            })
            .collect()
    }
}
