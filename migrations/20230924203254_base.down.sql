-- Add down migration script here
DROP TABLE excursions CASCADE;
DROP TABLE customers_types CASCADE;
DROP TABLE excursions_types;
DROP TABLE carts CASCADE;
DROP TABLE customers_type_costs CASCADE;
DROP TABLE cart_to_costs_types;
DROP TABLE payments;
DROP TABLE users;