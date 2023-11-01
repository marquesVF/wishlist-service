-- SQLite Compatible

-- Wishlist
create table Wishlist (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL
);

-- Product
create table Product (
  wishlist_id INTEGER REFERENCES Wishlist(id),
  sku VARCHAR NOT NULL,
  price_in_cents INTEGER NOT NULL,

  UNIQUE(wishlist_id, sku)
);

INSERT INTO Wishlist (name, user_id)
  VALUES
    ("My new kitchen", "user_01");
INSERT INTO Product (wishlist_id, sku, price_in_cents)
  VALUES
    (1, "13423-12231", 2344),
    (1, "99239-38383", 2000);

INSERT INTO Wishlist (name, user_id)
  VALUES
    ("Paintings", "anonymous_user_01");
INSERT INTO Product (wishlist_id, sku, price_in_cents)
  VALUES
    (2, "82908", 1220),
    (2, "23894", 9399),
    (2, "93231", 2040);
