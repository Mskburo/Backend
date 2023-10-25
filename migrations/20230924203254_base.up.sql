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
    route text NULL DEFAULT 'none',
    short_route text NULL DEFAULT 'none',
    meeting_info text NULL DEFAULT 'none',
    is_active boolean NULL DEFAULT true,
    times varchar(5) ARRAY NOT NULL,
    week_days integer NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT excursion_type_exursion_type_id_foreign FOREIGN KEY (excursion_type_id) REFERENCES excursions_types (id)
);
CREATE TABLE customers_type_costs (
    id serial NOT NULL,
    excursion_id integer NOT NULL,
    customers_types_id integer NOT NULL,
    cost float NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT customers_type_costs_customer_type_FK_customers_types_id_foreign FOREIGN KEY (customers_types_id) REFERENCES customers_types (id),
    CONSTRAINT customers_type_costs_excursion_FK_excursion_id_foreign FOREIGN KEY (excursion_id) REFERENCES excursions (id)
);
CREATE TABLE carts (
    id serial NOT NULL,
    date date NOT NULL,
    time varchar(5) NOT NULL,
    name text NOT NULL,
    tel varchar(20) NOT NULL,
    email text NOT NULL,
    bill text NOT NULL,
    created_at timestamptz NULL DEFAULT now(),
    is_paid boolean NULL DEFAULT false,
    PRIMARY KEY (id),
    CONSTRAINT proper_email CHECK (
        email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'
    )
);

CREATE TABLE payments (
    cart_id integer NOT NULL,
    payment_id varchar(40) NOT NULL,
    PRIMARY KEY (cart_id),
    CONSTRAINT payments_cart_id_carts_id_foreign FOREIGN KEY (cart_id) REFERENCES carts (id)
);

CREATE TABLE cart_to_costs_types (
    id serial NOT NULL,
    cart_id integer NOT NULL,
    customer_type_cost_id integer NOT NULL,
    amount integer NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT orders_to_tickets_order_id_orders_order_id_foreign FOREIGN KEY (cart_id) REFERENCES carts (id),
    CONSTRAINT orders_to_tickets_ticket_id_customers_type_costs_id_foreign FOREIGN KEY (customer_type_cost_id) REFERENCES customers_type_costs (id)
);

CREATE TABLE users (
    id serial NOT NULL,
    login varchar(100) NOT NULL,
    password varchar(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE OR REPLACE VIEW excursion_details AS
SELECT e.id AS id,
    et.name AS type_name,
    e.name AS name,
    e.description AS description,
    e.times AS times,
    e.short_description AS short_description,
    e.time AS time,
    e.available AS available,
    e.photo AS photo,
    e.route AS route,
    e.short_route AS short_route,
    e.meeting_info AS meeting_info,
    e.is_active AS is_active
FROM excursions e
    JOIN excursions_types et ON e.excursion_type_id = et.id;


INSERT INTO "excursions_types" ("id", "name")
VALUES (1, 'Автобусные'),
    (2, 'Автобусно-речные');



INSERT INTO "customers_types" ("id", "name")
VALUES (1, 'Дети'),
    (2, 'Взрослый'),
    (3, 'Взрослый');

INSERT INTO "excursions" (
        "id",
        "excursion_type_id",
        "name",
        "description",
        "short_description",
        "time",
        "available",
        "photo",
        "route",
        "short_route",
        "meeting_info",
        "is_active",
        "times",
        "week_days"
    )
VALUES (
        2,
        2,
        'Экс 2',
        'Описание большое 2',
        'Описание малое 2',
        '2 час',
        20,
        'excursion.png',
        'маршрут 2 - 2 - 23',
        'маршрут 2 - 23',
        'Митинг инфо 2',
        't',
        '{22:00,22:22,00:22}',
        135
    ),
    (
        1,
        1,
        'Экс 1',
        'Описание большое 1',
        'Описание малое 1',
        '1 час',
        30,
        'excursion.png',
        'маршрут 1 - 2 - 13',
        'маршрут 1 - 13',
        'Митинг инфо',
        't',
        '{12:00,16:00,19:00}',
        156
    );

INSERT INTO "customers_type_costs" (
        "id",
        "excursion_id",
        "customers_types_id",
        "cost"
    )
VALUES (1, 1, 1, 100),
    (2, 1, 1, 200),
    (3, 2, 1, 300),
    (4, 2, 2, 400);