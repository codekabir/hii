use actix_web::{get, post, web, App,HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct  SignUp{
    name:String,
    email:String,
    password:String,
}

#[get("/signup/{name}")]
async fn signup(path:web::Path<String>)->&'static str{

    
     dbg!(path.into_inner());
    "success"
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(signup)

    })

    .bind(("127.0.0.1",8080))?
    .run()
    .await

}