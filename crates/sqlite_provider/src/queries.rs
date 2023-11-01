pub const INSERT_WISHLIST: &str = "INSERT INTO Wishlist (name, user_id) VALUES (?1, ?2)";
pub const SELECT_WISHLIST_BY_USER_ID: &str =
    "SELECT id, name, user_id FROM Wishlist WHERE user_id = ?1";
pub const SELECT_WISHLIST: &str = "SELECT id, name, user_id FROM Wishlist WHERE id = ?1";
pub const SELECT_WISHLIST_PRODUCTS: &str = "
  SELECT
    sku, price_in_cents
  FROM
    Product
  WHERE
    Product.wishlist_id = ?1";
pub const INSERT_PRODUCT_INTO_WISHLIST: &str =
    "INSERT INTO Product (wishlist_id, sku, price_in_cents) VALUES (?1, ?2, ?3)";
