use crate::routes::utils::*;

#[post("/add_user")]
pub async fn add_user(
    user: web::Json<NewUser>, app_state: web::Data<AppState>
) -> HttpResponse {
    let added_user = sqlx::query(
        INSERT_USER
    ).bind(user.username.clone()).bind(user.email.clone())
    .execute(&app_state.pool).await;

    match added_user {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error adding new user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}

#[get("/get_user/{user_id}")]
pub async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: Result<Option<User>> = sqlx::query_as(
        SELECT_USER
    ).bind(user_id as u64)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(u) => HttpResponse::Ok().json(u.unwrap()),
        Err(e) => {
            eprintln!("Error getting user: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/check_user/{username}&{email}")]
pub async fn check_user(path: web::Path<(String, String)>, app_state: web::Data<AppState>
) -> HttpResponse {
    let (username, email) = path.into_inner();
    println!("URL - username: {username} - email: {email}");

    let user: Result<Option<User>> = sqlx::query_as(
        SELECT_USER_BY_CREDS
    ).bind(username).bind(email)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(u) => {
            match u {
                Some(u) => {
                    println!("U: {:#?}", u);
                    HttpResponse::Ok().json("Valid user")
                },
                None => HttpResponse::Ok().json("Invalid user")
            }
        },
        Err(e) => {
            eprintln!("Error getting user: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/get_all_users")]
pub async fn get_all_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users: Result<Vec<User>> = sqlx::query_as(
        SELECT_ALL_USERS
    ).fetch_all(&app_state.pool).await;

    match users {
        Ok(us) => HttpResponse::Ok().json(us),
        Err(e) => {
            eprintln!("Error getting all users: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[post("/update")]
pub async fn update_user(
    user: web::Form<User>, app_state: web::Data<AppState>
) -> HttpResponse {
    let updated: sqlx::Result<MySqlQueryResult> = sqlx::query(
        UPDATE_USER
    ).bind(user.username.clone()).bind(user.email.clone()).bind(user.id)
    .execute(&app_state.pool).await;

    match updated {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error updating user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}

#[get("/delete/{user_id}")]
pub async fn delete_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let deleted: sqlx::Result<MySqlQueryResult> = sqlx::query(
        DELETE_USER
    ).bind(user_id as u64).execute(&app_state.pool).await;

    match deleted {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error deleting user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}