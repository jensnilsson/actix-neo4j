use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use neo4rs::*;
use std::sync::Arc;
use uuid::Uuid;

async fn create(graph: web::Data<Arc<Graph>>, name: web::Path<String>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    let name = name.into_inner();
    let result = match graph.run(
        query("CREATE (p:Person {id: $id, name: $name})")
        .param("id", id.clone())
        .param("name", name.clone() )
    ).await {
        Ok(()) => {
            "OK"
        },
        Err(_) => {
            "Err"
        }
    };
    format!("Created node for: {}", &result)
}

async fn find(graph: web::Data<Arc<Graph>>, name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();
    let _result = graph.execute(
        query("MATCH (p:Person {name: $name}) RETURN p")
        .param("name", name.clone())
    ).await;
    format!("Found node for: {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = "localhost:7687";
    let user = "test";
    let pass = "test";
    
    let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
    HttpServer::new(move || {
        App::new()
            .data(graph.clone())
            .route("/create/{name}", web::get().to(create))
            .route("/find/{name}", web::get().to(find))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}