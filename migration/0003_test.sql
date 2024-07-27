INSERT INTO circles (name, super_circle_id) 
  VALUES ('Cortwo', NULL), ('Admin', 1), ('Mgmt', 1), ('Tech', 1), ('IT', 4), ('RND', 4);

INSERT INTO people (first_name)
  VALUES ('Nir'), ('Gil'), ('Pavan'), ('Dov'), ('John');

INSERT INTO organizations (name, circle_id)
  VALUES ('Cortwo', 1);

INSERT INTO services (name, organization_id) 
  VALUES ('Mail', 1), ('Admin Chat', 1);

INSERT INTO people_in_circle (person_id, circle_id)
  VALUES (1, 1), (2, 6), (3, 6), (4, 3), (4, 5);

INSERT INTO services_in_circle (service_id, circle_id)
  VALUES (1, 1), (2, 2);
