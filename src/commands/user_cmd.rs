use sqlx::PgPool;

use crate::{
    domain::{CreateUserFormData, RaabtaUser},
    user_db,
};

pub async fn create(user_form_data: CreateUserFormData, pool: &PgPool) -> Result<(), String> {
    let mut new_user: RaabtaUser = user_form_data.try_into()?;

    let mut student_inserted = false;
    for i in 1..=9 {
        let insert_user_result = user_db::insert_user(new_user.as_ref().into(), pool).await;
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

    let new_user_parent = new_user.create_parent_data()?;
    if let Some(new_user_parent) = new_user_parent {
        let student_user_id = new_user.id;
        let parent_user_id = new_user_parent.id;

        user_db::insert_user(new_user_parent.as_ref().into(), pool)
            .await
            .map_err(|e| e.to_string())?;
        user_db::set_student_parent_id(parent_user_id, student_user_id, pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
