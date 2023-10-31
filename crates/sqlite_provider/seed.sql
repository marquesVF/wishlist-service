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
INSERT INTO Product (sku, name, price_in_cents)
  VALUES
    ("354234", "Thyme & Table Stoneware Oval Serve Platter, Medallion", 1174),
    ("234342", "Corelle Cusco 16-piece Mugless Dinnerware Set, Service for 4", 4740),
    ("848484", "Zak Designs 12 Pieces Dinnerware Set Melamine Plastic Plates and Bowls", 3999),
    ("292383", "Loobuu Plastic Dinnerware Set of 16 Pieces", 2699),
    ("193211", "Lasko Cool Colors 20 Box Fan with 3-Speeds, B20200, White", 2399);

INSERT INTO Wishlist (name, user_id)
  VALUES
    ("My new kitchen", "vini");

INSERT INTO WishlistHasProducts (wishlist_id, product_sku)
  VALUES
    (1, "234342"),
    (1, "193211");
