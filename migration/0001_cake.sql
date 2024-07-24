CREATE TABLE cakes (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE fruits (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL,
    cake_id INT,
    FOREIGN KEY (cake_id) REFERENCES cakes(id)
);

CREATE TABLE fillings (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE cake_fillings (
    PRIMARY KEY (cake_id, filling_id),
    cake_id INT,
    filling_id INT,
    FOREIGN KEY (cake_id) REFERENCES cakes(id),
    FOREIGN KEY (filling_id) REFERENCES fillings(id)
);


