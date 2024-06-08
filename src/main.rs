use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Definindo uma estrutura genérica para nossos recursos
#[derive(Debug, Serialize, Deserialize)]
struct Resource<T> {
    id: usize,
    data: T,
}

// Estrutura de dados compartilhada entre as rotas
#[derive(Clone)]
struct AppState<T> {
    resources: Arc<Mutex<Vec<Resource<T>>>>,
}

// Rota para obter todos os recursos
async fn get_resources<T>(state: web::Data<AppState<T>>) -> impl Responder
where
    T: Serialize + 'static,
{
    let resources = state.resources.lock().unwrap();
    HttpResponse::Ok().json(&*resources)
}

// Rota para criar um novo recurso
async fn create_resource<T>(
    payload: web::Json<T>,
    state: web::Data<AppState<T>>,
) -> impl Responder
where
    T: Serialize + Deserialize<'static> + Clone + 'static,
{
    let mut resources = state.resources.lock().unwrap();
    let id = resources.len() + 1;
    let new_resource = Resource { id, data: payload.into_inner() };
    resources.push(new_resource);
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Criação de dados iniciais (pode ser um banco de dados, por exemplo)
    let initial_resources: Vec<Resource<String>> = vec![
        Resource { id: 1, data: "Resource 1".to_string() },
        Resource { id: 2, data: "Resource 2".to_string() },
    ];

    // Inicialização do estado da aplicação
    let app_state = web::Data::new(AppState {
        resources: Arc::new(Mutex::new(initial_resources)),
    });

    // Inicialização do servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Rotas
            .route("/resources", web::get().to(get_resources::<String>))
            .route("/resources", web::post().to(create_resource::<String>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
