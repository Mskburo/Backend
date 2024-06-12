BEGIN;
    ALTER TABLE carts ADD COLUMN promo_qr_id integer DEFAULT NULL;
    ALTER TABLE carts ADD CONSTRAINT carts_promo_qr_id_qrs_id_foreign FOREIGN KEY (promo_qr_id) REFERENCES qrs (id);
COMMIT;