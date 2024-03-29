-- Add migration script here
CREATE TABLE
    sys (
        name VARCHAR(255) NOT NULL,
        value VARCHAR(255) DEFAULT NULL,
        PRIMARY KEY (name)
    );

CREATE TABLE
    task (
        id CHAR(21) PRIMARY KEY,
        script VARCHAR(255) NOT NULL,
        cron VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        remark TEXT,
        max_executions INT,
        auto_start BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    header (
        id CHAR(21) PRIMARY KEY,
        topic VARCHAR(255) NOT NULL,
        qos INT NOT NULL
    );

CREATE TABLE
    message (
        id CHAR(21) PRIMARY KEY,
        topic VARCHAR(255) NOT NULL,
        qos INT NOT NULL,
        retain BOOLEAN NOT NULL,
        mine BOOLEAN NOT NULL,
        `timestamp` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
        sender VARCHAR(255),
        receiver VARCHAR(255),
        type VARCHAR(255) NOT NULL,
        raw TEXT NOT NULL,
        html TEXT NOT NULL,
        header_id CHAR(21) NOT NULL,
        FOREIGN KEY (header_id) REFERENCES header (id)
    );