use crate::services::BaseService;
use mongodb::{
    bson::{
        Document,
        doc,
    },
    Collection,
    error::Error,
    options::{
        FindOptions,
    },
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"), skip_serializing_if = "String::is_empty")]
    id: String,
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> User {
        User {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            password,
        }
    }
}
trait Copy {
    fn copy(&self) -> User;
}
impl Copy for User {
    fn copy(&self) -> User {
        User {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUserReq {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchUsersReq {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserReq {
    id: String,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteUserReq {
    pub id: String,
}

#[derive(Clone)]
pub struct UserService {
    base: BaseService<User>,
}

impl UserService {
    pub fn new(collection: Collection<User>) -> UserService {
        UserService {
            base: BaseService::new(collection),
        }
    }

    pub async fn create_user(&self, req: CreateUserReq) -> Result<User, Error> {
        let user= User::new(req.name, req.email, req.password);
        
        match self.base.insert(user.copy()).await {
            Ok(_) => Ok(user.copy()),
            Err(e) => Err(e),
        }
    }

    pub async fn search_users(&self, req: SearchUsersReq, options: Option<FindOptions>) -> Result<Vec<User>, Error> {
        let mut filter = Document::new();
        if let Some(name) = req.name {
            filter.insert("name", name);
        }
        if let Some(email) = req.email {
            filter.insert("email", email);
        }

        match self.base.find(filter, options).await {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }

    pub async fn update_user(&self, req: UpdateUserReq) -> Result<User, Error> {
        let filter = doc! {
            "_id": req.id.clone()
        };
        let mut update = Document::new();
        if let Some(name) = req.name {
            update.insert("name", name);
        }
        if let Some(email) = req.email {
            update.insert("email", email);
        }
        if let Some(password) = req.password {
            update.insert("password", password);
        }

        match self.base.update(filter.clone(), update).await {
            Ok(_) => {
                match self.base.find(filter.clone(), None).await {
                    Ok(users) => Ok(users[0].copy()),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub async fn delete_user(&self, req: DeleteUserReq) -> Result<User, Error> {
        let mut filter = Document::new();
        filter.insert("_id", req.id.clone());
        let user = match self.base.find(filter.clone(), None).await {
            Ok(users) => users[0].copy(),
            Err(e) => return Err(e),
        };

        match self.base.delete(filter).await {
            Ok(_) => Ok(user.copy()),
            Err(e) => Err(e),
        }
    }
}