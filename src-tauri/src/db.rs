use std::collections::BTreeMap;

use crate::bail;
use crate::prelude::*;

use surrealdb::sql::Id;
use surrealdb::sql::Thing;
use surrealdb::{
    sql::{parse, Object, Value},
    Datastore, Response, Session,
};

pub struct Db {
    pub store: Datastore,
    pub session: Session,
}

type Variables = Option<BTreeMap<String, Value>>;

pub trait TryFromObject: Sized {
    fn try_from_object(object: Object) -> Result<Self>;
}

impl TryFromObject for Thing {
    fn try_from_object(mut object: Object) -> Result<Self> {
        let Some(Value::Thing(th)) = object.remove("id") else {
            bail!("Failed to get id.");
        };

        Ok(th)
    }
}

impl TryFromObject for Id {
    fn try_from_object(object: Object) -> Result<Self> {
        Ok(Thing::try_from_object(object)?.id)
    }
}

impl TryFromObject for Object {
    fn try_from_object(object: Object) -> Result<Self> {
        Ok(object)
    }
}

impl Db {
    pub async fn new(
        path: &str,
        namespace: impl Into<String>,
        database: impl Into<String>,
    ) -> Result<Self> {
        let store = Datastore::new(path).await?;
        let session = Session::for_db(namespace.into(), database.into());

        Ok(Db { store, session })
    }

    pub async fn process(
        &self,
        query: surrealdb::sql::Query,
        vars: Variables,
    ) -> Result<Vec<Response>> {
        self.store
            .process(query, &self.session, vars, false)
            .await
            .map_err(Error::Surreal)
    }

    pub async fn execute(&self, query: &str, vars: Variables) -> Result<Vec<Response>> {
        let query = parse(query)?;
        self.process(query, vars).await
    }

    pub async fn get<T: TryFromObject>(&self, query: &str, vars: Variables) -> Result<T> {
        let ress = self.execute(query, vars).await?;

        let Some(res) = into_iter_of::<T>(ress)?.next().transpose()? else {
            bail!("Did not get a record.");
        };

        Ok(res)
    }

    pub async fn list<T: TryFromObject>(&self, query: &str, vars: Variables) -> Result<Vec<T>> {
        let ress = self.execute(query, vars).await?;
        into_iter_of::<T>(ress)?.collect()
    }
}

pub fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|res| res.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => bail!("A record was not an Object."),
            });

            Ok(it)
        }
        _ => bail!("No records found."),
    }
}

pub fn into_iter_of<T: TryFromObject>(
    ress: Vec<Response>,
) -> Result<impl Iterator<Item = Result<T>>> {
    Ok(into_iter_objects(ress)?.map(|res| res.and_then(T::try_from_object)))
}
