use bson::{oid::ObjectId, doc, Document};
use mongodb::{Collection, results::{InsertOneResult, UpdateResult, DeleteResult}, Cursor, options::FindOptions};
use serde::{Serialize, de::DeserializeOwned};

use crate::{common::errors::{AppResult, AppErrors}, models::tasks::Task};
use tokio_stream::StreamExt;


pub struct BaseService<T>{
    pub collection: Collection<T>,
    name: String
}
impl<T> BaseService<T> 
where T: DeserializeOwned + Unpin + Send + Sync + Serialize {

    pub fn new(collection:  Collection<T>, name: String) -> Self {
        Self {
            collection,
            name
        }
    }

    pub async fn create(&self, create_model: &T) -> AppResult<InsertOneResult> {
        let result = self.collection.insert_one(create_model, None).await;
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(AppErrors::InternalError(format!("Failed to create a new task: {}", e.to_string())))
        }
    }

    pub async fn get_one_by_id(&self, id: &ObjectId) -> AppResult<T> {
        let doc = doc! {"_id": id};
        self.get_one(doc).await
    }

    pub async fn get_one(&self, filter: Document) -> AppResult<T> {
        let data = self.collection.find_one(filter, None).await;

        let error_message = format!("Could not run find query on the data: {}", self.name);
        let data = AppErrors::from_unknown_result(data, &error_message)?;
        match data {
            Some(v) => Ok(v),
            None => Err(AppErrors::NotFound)
        }
    }


    pub async fn get_all_by_and_sort(&self, doc: Option<Document>, sort: Document) -> AppResult<Vec<T>> {
        let find_options = FindOptions::builder().sort(sort).build();
        let cursor = self.collection.find(doc, find_options).await;


        let cursor = AppErrors::from_unknown_result(cursor, "Failed to get task cursor")?;
        self.process_cursor(cursor).await
    }

    pub async fn process_cursor(
        &self,
        mut cursor: Cursor<T>,
    ) -> AppResult<Vec<T>> {
        let mut response_vec: Vec<T> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    response_vec.push(doc);
                }
                Err(e) => return Err(AppErrors::InternalError(e.to_string())),
            }
        }
        Ok(response_vec)
    }

    pub async fn get_all_by(&self, doc: Option<Document>) -> AppResult<Vec<T>> {
        let cursor = self.collection.find(doc, None).await;


        let cursor = AppErrors::from_unknown_result(cursor, "Failed to get task cursor")?;
        self.process_cursor(cursor).await
    }

    pub async fn update_by_doc(
        &self,
        filter: Document,
        fields: Document,
    ) -> AppResult<UpdateResult> {
        let doc = doc! {"$set": fields};
        match self.collection.update_one(filter, doc, None).await {
            Err(err) => Err(AppErrors::InternalError(err.to_string())),
            Ok(result) => Ok(result),
        }
    }

    pub async fn update_by_id(
        &self,
        id: &ObjectId,
        fields: Document,
    ) -> AppResult<UpdateResult> {
        let filter = doc! {"_id": id};
        self.update_by_doc(filter, fields).await
    }

    pub async fn delete_by_id(
        &self,
        id: &ObjectId,
    )  -> AppResult<DeleteResult> {
        let filter = doc! {"_id": id};
        match self.collection.delete_one(filter, None).await {
            Err(err) => Err(AppErrors::InternalError(err.to_string())),
            Ok(result) => Ok(result),
        }
    }

}