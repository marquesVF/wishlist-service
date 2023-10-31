use wishlist::Wishlist;

pub async fn create_wishlist(name: &str, user_id: &str) -> Wishlist {
    Wishlist {
        name: name.to_string(),
        user_id: user_id.to_string(),
        products: vec![],
    }
}

pub async fn find_user_wishlists(user_id: &str) -> Vec<Wishlist> {
    vec![create_wishlist("default", user_id).await]
}
