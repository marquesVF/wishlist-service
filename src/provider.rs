use sqlite_provider::SQLiteProvider;
use wishlist::WishlistProvider;

pub async fn wishlist_provider() -> Box<dyn WishlistProvider> {
    // TODO implement another provider. It could be connecting to a third party vendor
    let sqlite_provider = SQLiteProvider::new("data.db").await;

    Box::new(sqlite_provider)
}
