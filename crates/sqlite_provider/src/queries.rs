pub const CREATE_WISHLIST: &str = "INSERT INTO Wishlist (name, user_id) VALUES (?1, ?2)";
pub const SELECT_WISHLIST_BY_USER_ID: &str = "SELECT id, name, user_id FROM Wishlist WHERE user_id = ?1";
pub const SELECT_WISHLIST: &str = "SELECT id, name, user_id FROM Wishlist WHERE id = ?1";
pub const SELECT_WISHLIST_PRODUCTS: &str = "
  SELECT
    Product.name, Product.sku, Product.price_in_cents
  FROM
    Product
  JOIN
    WishlistHasProducts
  ON
    WishlistHasProducts.product_sku = Product.sku
  WHERE
    WishlistHasProducts.wishlist_id = ?1";
pub const INSERT_PRODUCT_INTO_WISHLIST: &str = "INSERT INTO WishlistHasProducts (wishlist_id, product_sku) VALUES (?1, ?2)";