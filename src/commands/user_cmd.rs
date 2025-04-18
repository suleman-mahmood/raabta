use anyhow::bail;
use sqlx::PgPool;

use crate::{
    domain::CreateUserFormData,
    user_db::{self, RaabtaUserCreateDTO},
};

pub async fn create(user_form_data: CreateUserFormData, pool: &PgPool) -> anyhow::Result<()> {
    let mut new_user: RaabtaUserCreateDTO = user_form_data.try_into()?;

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
                bail!(e);
            }
        }
    }

    if !student_inserted {
        log::error!("Couldn't insert student user into db because of non unique email");
        bail!("Choose a different first word in display name");
    }

    let new_user_parent = new_user.create_parent_data()?;
    if let Some(new_user_parent) = new_user_parent {
        let student_user_id = new_user.get_id();
        let parent_user_id = new_user_parent.get_id();

        user_db::insert_user(&new_user_parent, pool).await?;
        user_db::set_student_parent_id(parent_user_id, student_user_id, pool).await?;
    }

    Ok(())
}
