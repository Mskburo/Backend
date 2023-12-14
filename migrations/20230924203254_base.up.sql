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
CREATE TABLE qrs (
    id serial NOT NULL,
    name varchar(255) NOT NULL,
    count integer NOT NULL,
    PRIMARY KEY (id)
);
CREATE OR REPLACE FUNCTION increase_qr_count_by_id(p_id INT) RETURNS VOID AS $$ BEGIN
UPDATE qrs
SET count = count + 1
WHERE id = p_id;
END;
$$ LANGUAGE plpgsql;
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
    e.is_active AS is_active,
    e.week_days AS week_days
FROM excursions e
    JOIN excursions_types et ON e.excursion_type_id = et.id;

INSERT INTO "excursions_types" ("id", "name")
VALUES (1, 'Автобусная'),
    (2, 'Пешеходная'),
    (3, 'Комбинированная');


INSERT INTO "customers_types" ("id", "name")
VALUES (1, 'Взрослый'),
    (2, 'Пенсионер'),
    (3, 'Студент'),
    (4, 'Школьный'),
    (5, 'Многодетный');

    
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
        3,
        2,
        'Пешеходная экскурсия по центру Москвы',
        'Экскурсия в разработке. Скоро появится описание',
        'Экскурсия в разработке. Скоро появится описание',
        '2 часа',
        50,
        'steps.webp',
        'Путь уточняйте у экскурсовода',
        'Путь уточняйте у экскурсовода',
        'Место встречи уточняйте у экскурсовода ',
        'f',
        '{12:00}',
        1234567
    ),
    (
        4,
        3,
        'Автобусно-теплоходная экскурсия',
        'Прекрасная автобусно-водная обзорная экскурсия по Москве. Жемчужина наших туров! Начнем путешествие с ВДНХ, проедем по Проспекту Мира и Садовому кольцу, посетим парк “Зарядье” и Храма Христа Спасителя. 

Далее пересаживаемся на теплоход и совершаем путешествие по центру Москвы. На теплоходе мы проедем весь центр города, увидим такие красоты как Кремль, парк им. Горького, стадион Лужники, памятник Петру Первому, Министерство обороны и много другое.',
        'Прекрасная автобусно-водная обзорная экскурсия по Москве. Жемчужина наших туров!',
        '3.5 часа (час автобус, 2.5 теплоход)',
        35,
        'river.webp',
        'ВДНХ - Парк Зарядье - Котельническая набережная - Новоспасский причал',
        'ВДНХ - Новоспасский причал',
        '1-й Поперечный проезд. Выход №1 метро ВДНХ, повернуть налево и идти прямо 150 метров. Сбор группы напротив входа в музей космонавтики.
При возникновении трудностей уточняйте по номеру 8 (800) 444-21-37',
        'f',
        '{11:00,13:30,16:00}',
        12345
    ),
    (
        1,
        1,
        'Сердце столицы',
        'Серце столицы - самый популярный маршрут!

На этом маршруте мы собрали самые красивые и известные места Москвы. Каждое из этих мест значимо в историческом и культурном плане!
Мы начнем маршрут с ВДНХ, проедем по Проспекту Мира, дороге, которая играет значимую роль в нашей истории. 

Затем нас ждет красота парка “Зарядье”, где мы выйдем на небольшую прогулку. Храм Христа Спасителя, Воробьевы горы, Сталинские высотки и еще многое другое будет ждать вас на маршруте “Сердце Столицы”. Всю эту красоту вы увидите из окна комфортабельного автобуса, в котором есть кондиционер, мягкие и удобные кресла. А на протяжении всего маршрута вас будут сопровождать опытный гид, который имеет за плечами не одну экскурсию! 
',
        'На этом маршруте мы собрали самые красивые и известные места Москвы. Каждое из этих мест значимо в историческом и культурном плане!',
        '3 часа',
        35,
        'capitalHeart.webp',
        'ВДНХ - Зарядье - Храм Христа Спасителя - Воробьевы горы - МИД - ВДНХ',
        'ВДНХ - Воробьевы горы ',
        '1-й Поперечный проезд. Выход №1 метро ВДНХ, повернуть налево и идти прямо 150 метров. Сбор группы напротив входа в музей космонавтики.
При возникновении трудностей уточняйте по номеру 8 (800) 444-21-37',
        't',
        '{12:00,15:30}',
        357
    ),
    (
        2,
        3,
        'По Петровским местам',
        'Почувствуйте себя Императором России - мы поедем по местам где ездил, работал, жил и охотился Петр Первый. Этот маршрут позволит вам окунуться в историю и почувствовать дух того времени.

Наш гид расскажет про множество интересных исторических мест и как они связаны с Петром. Мы начнем маршрут с ВДНХ и проедем по Проспекту Мира, Садовом кольцу, Мясницкой улице. Далее сделаем остановку в парке “Зарядье”. Проедем по Яузским набережным, посетим Измайловский Кремль и еще много другое.
',
        'Почувствуйте себя Императором России - мы поедем по местам где ездил, работал, жил и охотился Петр Первый.',
        '3 часа',
        35,
        'petersPlaces.webp',
        'ВДНХ - Парк Зарядье - Садовое Кольцо - Бауманская - Семеновская - Измайлово - Преображенская площадь - Сокольнический парк - ВДНХ',
        'ВДНХ - Измайлово',
        '1-й Поперечный проезд. Выход №1 метро ВДНХ, повернуть налево и идти прямо 150 метров. Сбор группы напротив входа в музей космонавтики.
При возникновении трудностей уточняйте по номеру 8 (800) 444-21-37',
        't',
        '{12:00,15:30}',
        46
    );

INSERT INTO "customers_type_costs" (
        "id",
        "excursion_id",
        "customers_types_id",
        "cost"
    )
VALUES (9, 1, 1, 1000),
    (10, 1, 2, 900),
    (11, 1, 3, 900),
    (12, 1, 4, 600),
    (13, 1, 5, 600),
    (14, 2, 1, 1000),
    (15, 2, 2, 900),
    (16, 2, 3, 900),
    (17, 2, 4, 600),
    (18, 2, 5, 600);