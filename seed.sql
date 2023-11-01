-- SQLite Compatible

-- Products
create table Product (
  sku VARCHAR PRIMARY KEY,
  price_in_cents INTEGER NOT NULL
);

-- Wishlist
create table Wishlist (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL
);

-- WishlistHasProducts
create table WishlistHasProducts (
  wishlist_id INTEGER REFERENCES Wishlist(id) UNIQUE,
  product_sku VARCHAR REFERENCES Product(sku),
  UNIQUE(wishlist_id, product_sku)
);

-- Demo
INSERT INTO Product (sku, price_in_cents)
  VALUES
    ("1", 1174),
    ("2", 4740),
    ("3", 3999),
    ("4", 2699),
    ("5", 2399),
    ("6", 1174),
    ("7", 8320),
    ("8", 5040),
    ("9", 1000),
    ("10", 9399);

INSERT INTO Wishlist (name, user_id)
  VALUES
    ("My new kitchen", "vini");

INSERT INTO WishlistHasProducts (wishlist_id, product_sku)
  VALUES
    (1, "1"),
    (1, "2");
