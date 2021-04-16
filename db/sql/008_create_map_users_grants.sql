CREATE TABLE IF NOT EXISTS tbl_fact_map_grant_users(
    grant_id INT UNSIGNED,
    user_id INT UNSIGNED,
    FOREIGN KEY (grant_id)
        REFERENCES tbl_fact_grants(id),
    FOREIGN KEY (user_id)
        REFERENCES tbl_fact_users(id)
);
