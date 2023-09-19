CREATE TABLE customers_types (
  id serial NOT NULL,
  name varchar(50) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE excursions_types (
  id serial NOT NULL,
  name varchar(50) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE excursions (
  id serial NOT NULL,
  excursion_type_id integer NOT NULL,
  name varchar(100) NULL DEFAULT 'none',
  description text NULL DEFAULT 'none',
  short_description text NULL DEFAULT 'none',
  time varchar(50) NOT NULL,
  available integer NOT NULL,
  photo varchar(50) NOT NULL,
  route text NULL  DEFAULT 'none',
  short_route text NULL  DEFAULT 'none',
  meeting_info text NULL  DEFAULT 'none',
  is_active boolean NULL DEFAULT true,
  PRIMARY KEY (id),
  CONSTRAINT excursion_type_exursion_type_id_foreign FOREIGN KEY (excursion_type_id) REFERENCES excursions_types (id)
);

CREATE TABLE customers_type_costs (
  id integer NOT NULL,
  excursion_id integer NOT NULL,
  customers_types_id integer NOT NULL,
  cost float NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT customers_type_costs_customer_type_FK_customers_types_id_foreign FOREIGN KEY (customers_types_id) REFERENCES customers_types (id),
  CONSTRAINT customers_type_costs_excursion_FK_excursion_id_foreign FOREIGN KEY (excursion_id) REFERENCES excursions (id)
);
CREATE TABLE orders (
  id serial NOT NULL,
  excursion_id integer NOT NULL,
  date varchar(50) NOT NULL,
  time varchar(50) NOT NULL,
  name text NOT NULL,
  tel varchar(18) NOT NULL,
  email text NOT NULL,
  payment_type varchar(15) NOT NULL,
  bill text NOT NULL,
  created_at timestamptz NULL DEFAULT now(),
  is_paid boolean NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  CONSTRAINT orders_excursion_id_excursions_id_foreign FOREIGN KEY (excursion_id) REFERENCES excursions (id)
);

CREATE TABLE orders_to_tickets (
  id serial NOT NULL,
  order_id integer NOT NULL,
  customer_type_id integer NOT NULL,
  amount integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT orders_to_tickets_order_id_orders_order_id_foreign FOREIGN KEY (order_id) REFERENCES orders (id),
  CONSTRAINT orders_to_tickets_ticket_id_customers_type_costs_id_foreign FOREIGN KEY (customer_type_id) REFERENCES customers_type_costs (id)
);