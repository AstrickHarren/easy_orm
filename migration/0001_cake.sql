CREATE TABLE cake (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE fruit (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL,
    cake_id INT,
    FOREIGN KEY (cake_id) REFERENCES cake(id)
);

CREATE TABLE filling (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE cake_filling (
    PRIMARY KEY (cake_id, filling_id),
    cake_id INT,
    filling_id INT,
    FOREIGN KEY (cake_id) REFERENCES cake(id),
    FOREIGN KEY (filling_id) REFERENCES filling(id)
);


