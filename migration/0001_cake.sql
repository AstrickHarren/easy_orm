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

INSERT INTO cakes (name) VALUES ('CheeseCake'), ('ChocoCake');
INSERT INTO fillings (name) VALUES ('Cheese'), ('Chocolate'), ('Flour');
INSERT INTO cake_fillings (cake_id, filling_id) VALUES (1, 1), (2, 2), (1, 3), (2, 3);
INSERT INTO fruits (name, cake_id) VALUES ('Pineapple', 1), ('Grape', 2);


