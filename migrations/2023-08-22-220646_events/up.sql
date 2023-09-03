CREATE TABLE customers_types (
  id serial NOT NULL,
  name varchar(50) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE excursion_type (
  id serial NOT NULL,
  name varchar(50) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE excursion (
  id serial NOT NULL,
  excursion_type_id integer NOT NULL,
  name varchar(100) DEFAULT 'Empty',
  description text DEFAULT 'Empty',
  time varchar(50) NOT NULL,
  available integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT excursion_type_exursion_type_id_foreign FOREIGN KEY (excursion_type_id) REFERENCES excursion_type (id)
);

CREATE TABLE customers_type_costs (
  id serial NOT NULL,
  customers_types_id integer NOT NULL,
  cost float NOT NULL,
  excursion_id integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT customers_type_costs_customer_type_FK_customers_types_id_foreign FOREIGN KEY (customers_types_id) REFERENCES customers_types (id),
  CONSTRAINT customers_type_costs_excursion_FK_excursion_id_foreign FOREIGN KEY (excursion_id) REFERENCES excursion (id)
);

CREATE TABLE photos (
  id serial NOT NULL,
  path varchar(255) NOT NULL,
  excursion_id integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT photos_excursion_id_excursion_id_foreign FOREIGN KEY (excursion_id) REFERENCES excursion (id)
);

CREATE TABLE tickets (
  id serial NOT NULL,
  customers_type_costs_id integer NOT NULL,
  amount integer DEFAULT 0,
  PRIMARY KEY (id),
  CONSTRAINT tickets_cost_FK_customers_type_costs_id_foreign FOREIGN KEY (customers_type_costs_id) REFERENCES customers_type_costs (id)
);

INSERT INTO "excursion_type" ("name")
VALUES ('Стандартный тариф');