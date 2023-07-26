DROP TABLE IF EXISTS todo_item;
DROP TABLE IF EXISTS todo_list;

CREATE TABLE todo_list (
    id serial PRIMARY KEY,
    title VARCHAR(150) NOT NULL
);

CREATE TABLE todo_item (
    id serial PRIMARY KEY,
    title VARCHAR(150)NOT NULL,
    checked boolean NOT NULL DEFAULT false,
    list_id integer NOT NULL,
    FOREIGN KEY (list_id) REFERENCES todo_list(id)
);

INSERT INTO todo_list (title) VALUES('List 1'), ('List 2');

INSERT INTO todo_item ( title, list_id ) VALUES
('Item 1', 1), ('Item 2', 1),
('Item 1', 2), ('Item 3', 2)
;