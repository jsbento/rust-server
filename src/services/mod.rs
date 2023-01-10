use mongodb:: {
    bson::Document,
    Collection,
    error::Error,
    options:: {
        UpdateModifications,
        FindOptions,
    },
    results::{
        DeleteResult,
        InsertOneResult,
        UpdateResult,
    },
};
use tokio_stream::StreamExt;

pub mod user_service;

#[derive(Clone)]
pub struct BaseService<T> {
    collection: Collection<T>,
}

impl<T> BaseService<T> {
    pub fn new(collection: Collection<T>) -> BaseService<T> {
        BaseService {
            collection,
        }
    }

    pub async fn insert(&self, document: T) -> Result<InsertOneResult, Error>
        where T: serde::Serialize
    {
        self.collection.insert_one(document, None).await
    }

    pub async fn update(&self, filter: Document, update: Document) -> Result<UpdateResult, Error> {
        self.collection.update_one(filter, update, None).await
    }

    pub async fn delete(&self, filter: Document) -> Result<DeleteResult, Error> {
        self.collection.delete_one(filter, None).await
    }

    pub async fn find(&self, filter: Document, opts: Option<FindOptions>) -> Result<Vec<T>, Error>
        where T: serde::de::DeserializeOwned + Sync + Send + Unpin
    {
        let cursor = self.collection.find(filter, opts).await.unwrap();
        cursor.collect().await
    }

    pub async fn aggregate(&self, pipe: Vec<Document>) -> Result<Vec<Document>, Error>
        where T: serde::de::DeserializeOwned
    {
        let cursor = self.collection.aggregate(pipe, None).await.unwrap();
        cursor.collect().await
    }
}

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_svc: user_service::UserService,
}
