-- Create a new database for the microservice
CREATE DATABASE wishlistdb;

-- Switch to the newly created database
\c wishlistdb;

-- Products
create table Product (
  sku VARCHAR PRIMARY KEY,
  price_in_cents INTEGER NOT NULL,
  name TEXT NOT NULL
);

-- Wishlist
create table Wishlist (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL
);

-- WishlistHasProducts
create table WishlistHasProducts (
  wishlist_id INTEGER REFERENCES Wishlist(id),
  product_sku VARCHAR REFERENCES Product(sku),
  UNIQUE(wishlist_id, product_sku)
);

