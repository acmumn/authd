CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR NOT NULL,
	realname VARCHAR,
	username VARCHAR NOT NULL,
	passhash VARCHAR NOT NULL,
	cardhash VARCHAR,
	cardlast CHAR(4))
