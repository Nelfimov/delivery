use diesel::PgConnection;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::update;
use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::kernel::location::Location;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::courier_repository_port::GetAllCouriersResponse;
use ports::errors::RepositoryError;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::ptr::NonNull;
use uuid::Uuid;

use crate::courier::courier_mapper::CourierRecord;
use crate::courier::courier_schema::couriers::dsl::id;
use crate::courier::courier_schema::couriers::dsl::location_x;
use crate::courier::courier_schema::couriers::dsl::location_y;
use crate::courier::courier_schema::couriers::dsl::name;
use crate::courier::courier_schema::couriers::dsl::*;
use crate::courier::courier_schema::couriers::table;
use crate::errors::postgres_error::PostgresError;
use crate::storage_place::storage_place_dto::StoragePlaceDto;
use crate::storage_place::storage_place_schema::storage_places::dsl::*;
use crate::storage_place::storage_place_schema::storage_places::order_id;

use super::courier_dto::CourierDto;

pub struct CourierRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
    shared_connection: Option<NonNull<PgConnection>>,
}

// SAFETY: `shared_connection` is only accessed via `&mut self`, preventing cross-thread use.
unsafe impl Send for CourierRepository {}

impl CourierRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool,
            shared_connection: None,
        }
    }

    pub fn with_shared_connection(
        pool: Pool<ConnectionManager<PgConnection>>,
        connection: NonNull<PgConnection>,
    ) -> Self {
        Self {
            pool,
            shared_connection: Some(connection),
        }
    }

    fn connection(&mut self) -> Result<RepositoryConn<'_>, RepositoryError> {
        if let Some(conn_ptr) = self.shared_connection {
            // SAFETY: conn_ptr originates from a live transaction connection and
            // is only accessed synchronously within the transaction scope.
            let conn = unsafe { &mut *conn_ptr.as_ptr() };
            return Ok(RepositoryConn::Borrowed(conn));
        }

        let conn = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        Ok(RepositoryConn::Pooled(conn))
    }
}

impl CourierRepositoryPort for CourierRepository {
    fn add(&mut self, c: Courier) -> Result<(), RepositoryError> {
        let courier_dto: CourierDto = c.clone().into();
        let storage_places_dto: Vec<StoragePlaceDto> = c
            .storage_places()
            .to_owned()
            .into_iter()
            .map(|f| StoragePlaceDto::from_dto(f, courier_dto.id))
            .collect();

        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        conn.transaction(|tx| {
            insert_into(couriers).values(&courier_dto).execute(tx)?;

            insert_into(storage_places)
                .values(storage_places_dto)
                .execute(tx)?;

            diesel::result::QueryResult::Ok(())
        })
        .map_err(PostgresError::from)?;

        Ok(())
    }

    fn update(&mut self, c: Courier) -> Result<(), RepositoryError> {
        let courier_dto: CourierDto = c.clone().into();
        let storage_places_dto: Vec<StoragePlaceDto> = c
            .storage_places()
            .to_owned()
            .into_iter()
            .map(|f| StoragePlaceDto::from_dto(f, courier_dto.id))
            .collect();
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        conn.transaction(|tx| {
            update(couriers.find(courier_dto.id))
                .set(&courier_dto)
                .execute(tx)?;

            if !storage_places_dto.is_empty() {
                for sp in storage_places_dto {
                    update(storage_places.find(sp.id)).set(&sp).execute(tx)?;
                }
            }

            diesel::result::QueryResult::Ok(())
        })
        .map_err(PostgresError::from)?;

        Ok(())
    }

    fn get_by_id(&mut self, c_id: CourierId) -> Result<Courier, RepositoryError> {
        let mut connection = self.connection()?;

        let results: Vec<(CourierDto, StoragePlaceDto)> = couriers
            .inner_join(storage_places)
            .filter(id.eq(c_id.0))
            .load(connection.as_mut())
            .map_err(PostgresError::from)?;

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
        let mut connection = self.connection()?;

        let rows: Vec<(CourierDto, StoragePlaceDto)> = couriers
            .inner_join(storage_places)
            .filter(order_id.is_null())
            .load(connection.as_mut())
            .map_err(PostgresError::from)?;

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

    fn get_all_couriers(&mut self) -> Result<Vec<GetAllCouriersResponse>, RepositoryError> {
        let mut connection = self.connection()?;

        let rows = table
            .select((id, name, location_x, location_y))
            .load::<(Uuid, String, i16, i16)>(connection.as_mut())
            .map_err(PostgresError::from)?;

        let result: Vec<GetAllCouriersResponse> = rows
            .iter()
            .filter_map(|v| {
                let location = Location::new(v.2 as u8, v.3 as u8);

                match location {
                    Ok(l) => Some(GetAllCouriersResponse {
                        id: CourierId(v.0),
                        name: CourierName(v.1.clone()),
                        location: l,
                    }),
                    Err(e) => {
                        println!("{}", e);
                        None
                    }
                }
            })
            .collect();

        Ok(result)
    }
}

enum RepositoryConn<'a> {
    Borrowed(&'a mut PgConnection),
    Pooled(PooledConnection<ConnectionManager<PgConnection>>),
}

impl<'a> RepositoryConn<'a> {
    fn as_mut(&mut self) -> &mut PgConnection {
        match self {
            RepositoryConn::Borrowed(conn) => conn,
            RepositoryConn::Pooled(conn) => conn.deref_mut(),
        }
    }
}
