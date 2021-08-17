CREATE TABLE IF NOT EXISTS tbl_fact_grants(
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        grant_number VARCHAR(255),
        title VARCHAR(255),
        source_id INT UNSIGNED,
        original_amt FLOAT(7,2),
        balance FLOAT(7,2),
        status ENUM('PENDING', 'APPROVED', 'DENIED'),
        administrator_id INT UNSIGNED,
        FOREIGN KEY (source_id)
                REFERENCES tbl_fact_granting_entity(id),
        FOREIGN KEY (administrator_id)
                REFERENCES tbl_fact_users(id)
);
