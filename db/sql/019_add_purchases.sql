INSERT INTO  tbl_fact_purchases (grant_id, name, qty, url, unit_cost, total_cost, purchaser_id, order_date, deliver_date, cancel_date)
VALUES 
    ( 1, 'MacBook Pro', 1, 'https://apple.com', 2499, 2500, 1, '2021-04-10', NULL,  NULL ),
    ( 2, 'Pencils', 100, 'https://staples.com',  100,  10.49,  2,  '2021-04-01', NULL, NULL ),
    ( 3, 'Lab Coats', 25, 'http://evilcorp.com',  2500, 2750.99, 3, '2021-01-02', '2021-01-24', NULL ),
    ( 4, 'Supports Lunch', 5, 'https://tasteunlimited.com',  350, 375.45, 4, '2021-02-14', '2021-02-14', NULL ),
    ( 5, 'TA Conference tickets', 10, 'https://hooli.com',  5500, 5750,  5, '2021-02-18', '2021-02-18', NULL );
