CREATE TABLE IF NOT EXISTS tbl_fact_users(
        id INT UNSIGNED AUTO_INCREMENT NOT NULL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL,
        created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
        user_role ENUM('FACULTY', 'ADMINISTRATOR'),
        department INT UNSIGNED NOT NULL,
        FOREIGN KEY (department)
                REFERENCES tbl_fact_departments(id)
);
