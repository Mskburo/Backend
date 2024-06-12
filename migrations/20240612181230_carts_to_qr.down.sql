BEGIN;
    ALTER TABLE carts DROP CONSTRAINT carts_promo_qr_id_qrs_id_foreign;
    ALTER TABLE carts DROP COLUMN promo_qr_id;
COMMIT;
