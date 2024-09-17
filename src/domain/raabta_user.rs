enum RaabtaUser {
    Student(Student),
    Parent(UserData),
    Teacher(UserData),
    SchoolAdmin(SchoolAdmin),
}

struct UserData {
    id: String,
    display_name: String,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    created_at: String,
    updated_at: String,
}

struct Student {
    uesr_data: UserData,
    parent_user_id: String,
}

struct SchoolAdmin {
    id: String,
    display_name: String,
}
