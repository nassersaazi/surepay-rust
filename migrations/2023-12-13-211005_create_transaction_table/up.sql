-- Your SQL goes here
CREATE TABLE transactions
(
    transactionId          VARCHAR(255) PRIMARY KEY,
    accountNumber       BIGINT NOT NULL,
    accountName          VARCHAR(255),
    accountCategory          VARCHAR(255),
    accountProvider          VARCHAR(255),
    bankCode          VARCHAR(255),
    password          VARCHAR(255),
    tranAmount          INT,
    tranCategory          VARCHAR(255),
    channel          VARCHAR(255),
    currency          VARCHAR(255),
    paymentDate  TIMESTAMP,
    tranSignature          VARCHAR(255),
    narration          VARCHAR(255)
)