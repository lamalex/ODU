CREATE TABLE IF NOT EXISTS tbl_fact_purchases(
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        grant_id INT UNSIGNED,
        name VARCHAR(255),
        qty INT UNSIGNED,
        url VARCHAR(255),
        unit_cost FLOAT(7,2),
        total_cost FLOAT(7,2),
        purchaser_id INT UNSIGNED,
        order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        deliver_date TIMESTAMP,
        cancel_date TIMESTAMP,
        FOREIGN KEY (grant_id)
                REFERENCES tbl_fact_grants(id),
        FOREIGN KEY (purchaser_id)
                REFERENCES tbl_fact_users(id)
);
