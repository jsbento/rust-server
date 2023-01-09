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

    pub async fn update(&self, filter: Document, update: T) -> Result<UpdateResult, Error>
        where T: Into<UpdateModifications>
    {
        self.collection.update_one(filter, update, None).await
    }

    pub async fn delete(&self, filter: Document) -> Result<DeleteResult, Error> {
        self.collection.delete_one(filter, None).await
    }

    pub async fn find(&self, filter: Document, opts: Option<FindOptions>) -> Vec<T>
        where T: serde::de::DeserializeOwned + Sync + Send + Unpin
    {
        let cursor = self.collection.find(filter, opts).await.unwrap();
        match cursor.collect().await {
            Ok(results) => results,
            Err(error) => {
                println!("Error: {}", error);
                Vec::new()
            }
        }
    }

    pub async fn aggregate(&self, pipe: Vec<Document>) -> Vec<Document>
        where T: serde::de::DeserializeOwned
    {
        let cursor = self.collection.aggregate(pipe, None).await.unwrap();
        match cursor.collect().await {
            Ok(results) => results,
            Err(_) => Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_svc: user_service::UserService,
}
