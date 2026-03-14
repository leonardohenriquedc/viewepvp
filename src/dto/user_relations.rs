pub struct UserRelations {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub groups: Vec<GroupRelations>,
}

pub struct GroupRelations {
    pub id: i32,
    pub name: String,
    pub role: RoleRelations,
}

pub struct RoleRelations {
    pub id: i32,
    pub role: String,
}
