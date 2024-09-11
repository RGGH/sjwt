use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
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

    println!("Scope created successfully!");


        // Get user input
    let (email, password) = get_user_credentials().await.unwrap();

    // sign up to get JWT
    let jwt = db
        .signup(Scope {
            namespace: "test",
            database: "test",
            scope: "admin",
            params: Credentials {
                email : &email,
                pass: &password,
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

async fn get_user_credentials() -> io::Result<(String, String)> {
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter email:");
    io::stdin().read_line(&mut email)?;
    let email = email.trim().to_string(); // Remove any extra whitespace/newlines

    println!("Enter password:");
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string(); // Remove any extra whitespace/newlines

    Ok((email, password))
}
