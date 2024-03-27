-- Add migration script here
CREATE TABLE
    sys (
        name VARCHAR(255) NOT NULL,
        value VARCHAR(255) DEFAULT NULL,
        PRIMARY KEY (name)
    );

CREATE TABLE
    tasks (
        id INT AUTO_INCREMENT PRIMARY KEY,
        script VARCHAR(255) NOT NULL,
        cron VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        remark TEXT,
        max_executions INT,
        auto_start BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    messages (
        id CHAR(21) PRIMARY KEY,
        topic VARCHAR(255) NOT NULL,
        qos INT NOT NULL,
        retain BOOLEAN NOT NULL,
        mine BOOLEAN NOT NULL,
        timestamp BIGINT NOT NULL,
        content BLOB NOT NULL,
        sender VARCHAR(255),
        receiver VARCHAR(255)
    );