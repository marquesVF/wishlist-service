-- SQLite

-- Products
create table Product (
  sku VARCHAR PRIMARY KEY,
  price_in_cents INTEGER NOT NULL,
  name TEXT NOT NULL
);

-- Wishlist
create table Wishlist (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL
);

-- WishlistHasProducts
create table WishlistHasProducts (
  wishlist_id INTEGER REFERENCES Wishlist(id),
  product_sku INTEGER REFERENCES Product(sku)
);

-- Demo
INSERT INTO Product (sku, price_in_cents, name)
  VALUES
    ("1", 1174, "Thyme & Table Stoneware Oval Serve Platter, Medallion"),
    ("2", 4740, "Corelle Cusco 16-piece Mugless Dinnerware Set, Service for 4"),
    ("3", 3999, "Zak Designs 12 Pieces Dinnerware Set Melamine Plastic Plates and Bowls"),
    ("4", 2699, "Loobuu Plastic Dinnerware Set of 16 Pieces"),
    ("5", 2399, "Lasko Cool Colors 20 Box Fan with 3-Speeds, B20200, White");

INSERT INTO Wishlist (name, user_id)
  VALUES
    ("My new kitchen", "vini");

INSERT INTO WishlistHasProducts (wishlist_id, product_sku)
  VALUES
    (1, "234342"),
    (1, "193211");
