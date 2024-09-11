use serde::Serialize;
use std::fs::File;
use std::io::Write;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Root, Scope};
use surrealdb::Response;
use surrealdb::Surreal;

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}

async fn create_scope(db: &Surreal<Client>) -> surrealdb::Result<Response> {
    let query = r#"
        DEFINE SCOPE admin
            SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
            SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) );
    "#;

    db.query(query).await
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.use_ns("test").use_db("test").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Create the scope for the user
    create_scope(&db).await?;

    // sign up to get JWT
    let jwt = db
        .signup(Scope {
            namespace: "test",
            database: "test",
            scope: "admin",
            params: Credentials {
                email: "info@surrealdb.com",
                pass: "123456",
            },
        })
        .await?;

    let token = jwt.as_insecure_token();
    println!("JWT Token: {:?}", token);

    // Write the token to a file
    let file = File::create("token.txt");
    match writeln!(file.expect("file"), "{}", token) {
        Ok(_) => println!("Token has been written to token.txt"),
        Err(e) => eprintln!("Failed to write token to file: {}", e),
    }

    // Perform a query
    let query = r#"
        SELECT * FROM user;
    "#;

    let response = perform_query(&db, query).await?;
    println!("Query Response: {:?}", response);

    Ok(())
}

// Function to perform a query
async fn perform_query(db: &Surreal<Client>, query: &str) -> surrealdb::Result<Response> {
    let response = db.query(query).await?;
    Ok(response)
}
