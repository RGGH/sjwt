use serde::Serialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Root, Scope};
use surrealdb::Surreal;
use surrealdb::Response;



// Struct for credentials
#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}

// Function to create a scope
async fn create_scope(db: &Surreal<Client>) -> surrealdb::Result<Response> {
    let query = r#"
        DEFINE SCOPE user
            SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
            SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) );
    "#;

    db.query(query).await
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create a new WebSocket connection client to SurrealDB
    // dont use ws:// !!!
    //let db = Surreal::new::<Ws>("ws://localhost:8000").await?;
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Select the namespace and database
    db.use_ns("test").use_db("test").await?;

    // Sign in as root
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Create the scope for the user
    create_scope(&db).await?;

    println!("Scope created successfully!");

    // Sign up a new user
    let jwt = db
        .signup(Scope {
            namespace: "test",
            database: "test",
            scope: "user",
            params: Credentials {
                email: "info@surrealdb.com",
                pass: "123456",
            },
        })
        .await?;

    // Get the token from the JWT
    let token = jwt.as_insecure_token();
    println!("JWT Token: {:?}", token);

    Ok(())
}
