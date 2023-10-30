use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct Wishlist {
    pub user_id: String,
    pub name: String,
}
