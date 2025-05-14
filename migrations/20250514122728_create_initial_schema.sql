-- Add migration script here
CREATE TABLE suppliers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE expenses (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    supplier_id INTEGER NOT NULL REFERENCES suppliers(id),
    amount REAL NOT NULL,
    date TEXT NOT NULL, -- ISO8601 string for chrono::NaiveDate
    submitter_id INTEGER NOT NULL REFERENCES users(id),
    status TEXT NOT NULL -- must match your Rust enum (e.g. 'Pending')
);

INSERT INTO suppliers (id, name) VALUES
(1, 'Supplier A'),
(2, 'Supplier B');

INSERT INTO users (id, name) VALUES
(1, 'Alice'),
(2, 'Bob');

INSERT INTO expenses (id, description, supplier_id, amount, date, submitter_id, status) VALUES
(1, 'Office Supplies', 1, 150.00, '2023-05-01', 1, 'Pending'),
(2, 'Travel Expenses', 2, 300.00, '2023-05-02', 2, 'Approved'),
(3, 'Software License', 1, 1200.00, '2023-05-03', 1, 'Paid');