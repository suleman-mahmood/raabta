use sqlx::PgPool;

use crate::admin_portal::{user_db, CreateUser, CreateUserFormData, UserRole};

pub async fn create_user(user_form_data: CreateUserFormData, pool: &PgPool) -> Result<(), String> {
    let mut new_user: CreateUser = user_form_data.try_into()?;

    let mut student_inserted = false;
    for i in 1..=9 {
        let insert_user_result = user_db::insert_user(&new_user, pool).await;
        match insert_user_result {
            Ok(_) => {
                student_inserted = true;
                break;
            }
            Err(e) => {
                if e.as_database_error().unwrap().is_unique_violation() {
                    new_user.regenerate_email(i);
                    continue;
                }
                log::error!("Couldn't insert student user into db: {:?}", e);
                return Err(e.to_string());
            }
        }
    }
    if !student_inserted {
        log::error!("Couldn't insert student user into db because of non unique email");
        return Err("Choose a different first word in display name".to_string());
    }

    match new_user.user_role {
        UserRole::Teacher => Ok(()),
        UserRole::Student => {
            let new_user_parent = CreateUser::create_parent_data(&new_user);
            let student_user_id = new_user.id;
            let parent_user_id = new_user_parent.id;

            user_db::insert_user(&new_user_parent, pool)
                .await
                .map_err(|e| e.to_string())?;
            user_db::set_student_parent_id(parent_user_id, student_user_id, pool)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err(format!(
            "Unknown user role {} for newly created user",
            new_user.user_role
        )),
    }
}
