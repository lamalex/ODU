CREATE TABLE IF NOT EXISTS tbl_fact_students (
        uin INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255),
        department_id INT UNSIGNED,
        program ENUM('UG', 'MS', 'PHD'),
        FOREIGN KEY (department_id)
                REFERENCES tbl_fact_departments(id)
);
