use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
pub struct User{
    pub id :i32,
    pub usernmae:String,
}

#[derive(Debug,sqlx::FromRow)]
pub struct UserWithHash{
    pub id:i32,
    pub password_hash:String
} 


#[derive(Deserialize)]
pub struct RegisterRequest{
    pub username:String,
    pub password:String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    pub username:String,
    pub password:String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub token:String
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Nid{
    pub id_no:String,
    pub name:String,
    pub age:i32,
    pub addr:String,
    pub father_name:String,
    pub mother_name:String,
}