-- This file should undo anything in `up.sql`
DROP TABLE excursions CASCADE;
DROP TABLE customers_types CASCADE;
DROP TABLE excursions_types;
DROP TABLE carts CASCADE;
DROP TABLE customers_type_costs CASCADE;
DROP TABLE cart_to_costs_types;