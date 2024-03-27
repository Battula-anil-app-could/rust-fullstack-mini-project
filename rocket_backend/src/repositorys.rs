use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::modules::*;
use crate::schema::*;


pub struct RustaceanRepository;

impl RustaceanRepository{
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean>{
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>>{
        rustaceans::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_rustacean: NewRustancean) -> QueryResult<Rustacean>{
        diesel::insert_into(rustaceans::table)
                .values(new_rustacean)
                .get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(&rustacean.name), 
                rustaceans::email.eq(&rustacean.email) 
            ))
            .get_result(c) 
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(rustaceans::table.find(id))
        .execute(c)
        .await
    }
}


pub struct CratesRepository;

impl CratesRepository{
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crates>{
        crates::table.find(id).get_result(c).await
    }

    pub async fn find_all(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crates>>{
        crates::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_rate: NewCreate) -> QueryResult<Crates>{
        diesel::insert_into(crates::table)
        .values(new_rate).get_result(c).await
    }

    pub async fn update(c: &mut AsyncPgConnection,id: i32,  a_crates:Crates) -> QueryResult<Crates>{
        diesel::update(crates::table.find(id))
        .set((
            crates::rustacean_id.eq(a_crates.rustacean_id),
            crates::name.eq(a_crates.name),
            crates::code.eq(a_crates.code),
            crates::version.eq(a_crates.version),
            crates::description.eq(a_crates.description)
        ))
        .get_result(c)
        .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(crates::table.find(id))
        .execute(c)
        .await
    }

}

pub struct UserRepository;
pub struct RolesRepository;

impl UserRepository{

pub async fn find_by_user_name(c: &mut AsyncPgConnection, username: &String) -> QueryResult<User>{
    users::table.filter(users::username.eq(username)).first(c).await
}
    pub async fn list_of_users(c: &mut AsyncPgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>>{
        let user = users::table.load::<User>(c).await?;
        
        let result = user_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c).await?
            .grouped_by(&user);
        
        Ok(user.into_iter().zip(result).collect())
        
    }
    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser, role_codes: Vec<String>) -> QueryResult<User>{
        let user = diesel::insert_into(users::table)
        .values(new_user).get_result::<User>(c).await?;

        for role_code in role_codes{
            let new_user_role = {
                if let Ok(role) = RolesRepository::finde_by_role_code(c, role_code.to_owned()).await{
                    NeewUserRole{user_id: user.id, role_id: role.id}
    
               }else{
                    let new_role = NewRole{code:role_code.to_owned(), name: role_code.to_owned()};
                    let role = RolesRepository::create(c, new_role).await?;
                    NeewUserRole{user_id: user.id, role_id: role.id}
               }
            };

            diesel::insert_into(user_roles::table)
            .values(new_user_role).get_result::<UserRole>(c).await?;
           
        }
        
        Ok(user)
    }

    pub async fn delete_user(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(
            user_roles::table.filter(user_roles::user_id.eq(id)))
            .execute(c).await?;
        diesel::delete(users::table.find(id)).execute(c).await
    }
}

impl RolesRepository{
    pub async fn find_by_role_id(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>>{
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }
    pub async fn finde_by_role_code(c: &mut AsyncPgConnection, role_code: String) -> QueryResult<Role>{
        roles::table.filter(roles::code.eq(role_code)).first(c).await
    }
    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>>{
        let user_roles = UserRole::belonging_to(&user).get_results::<UserRole>(c).await?;
        
        let role_ids: Vec<i32> = user_roles.iter().map(|user_role: &UserRole| user_role.role_id).collect();
        
        Self::find_by_role_id(c, role_ids).await
    }
    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role>{
        diesel::insert_into(roles::table)
        .values(new_role).get_result(c).await
    }
}