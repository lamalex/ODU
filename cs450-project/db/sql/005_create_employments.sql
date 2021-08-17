CREATE TABLE IF NOT EXISTS tbl_fact_employments(
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        student_uin INT UNSIGNED,
        type ENUM('GRA', 'GTA', 'URA'),
        semester VARCHAR(255),
        faculty_id INT UNSIGNED,
        amount FLOAT(7,0),
        payment_type ENUM('STIPEND', 'SALARY'),
        workload SMALLINT UNSIGNED,
        start_date TIMESTAMP,
        end_date TIMESTAMP,
        FOREIGN KEY (student_uin)
                REFERENCES tbl_fact_students(uin),
        FOREIGN KEY (faculty_id)
                REFERENCES tbl_fact_users(id)
);
