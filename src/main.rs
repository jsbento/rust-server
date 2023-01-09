use actix_web::{
    get,
    post,
    put,
    delete,
    web,
    App,
    HttpServer,
    HttpResponse,
    middleware::Logger,
};
use mongodb::{
    Client,
};
use dotenv;

mod services;
use services::{
    user_service::{
        CreateUserReq,
        SearchUsersReq,
        User,
        UserService,
    },
    ServiceContainer
};

fn init_services(client: &Client) -> ServiceContainer {
    let db = client.database("rust-server");
    let user_collection = db.collection::<User>("users");

    ServiceContainer {
        user_svc: UserService::new(user_collection),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mongo_uri = dotenv::var("MONGO_URI").unwrap();
    let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB");
    let services = init_services(&client);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(services.clone()))
            .service(healthz)
            .service(create_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/healthz")]
async fn healthz() -> HttpResponse {
    HttpResponse::Ok().body("Server is up and running!")
}

#[post("/users")]
async fn create_user(services: web::Data<ServiceContainer>, req: web::Json<CreateUserReq>) -> HttpResponse {
    let user = req.into_inner();
    let result = services.user_svc.create_user(user).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users")]
async fn get_users(services: web::Data<ServiceContainer>, req: web::Query<SearchUsersReq>) -> HttpResponse {
    let users = services.user_svc.search_users(req.into_inner(), None).await;
    HttpResponse::Ok().json(users)
}
