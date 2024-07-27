CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE circles (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  is_connected BOOLEAN DEFAULT TRUE NOT NULL,
  super_circle_id BIGINT references circles(id),
  admin_circle_id BIGINT references circles(id)
);

CREATE TABLE people (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

  first_name VARCHAR NOT NULL, 
  last_name VARCHAR, 
  external_identity_provider VARCHAR,
  external_identity_number VARCHAR,
  email VARCHAR,
  phone_number VARCHAR,
  address_line_1 VARCHAR,
  address_line_2 VARCHAR,
  city VARCHAR,
  state_or_province VARCHAR,
  postal_code VARCHAR,
  country VARCHAR
);

CREATE TABLE organizations (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  circle_id BIGINT references circles(id) NOT NULL,

  website VARCHAR,
  external_identity_provider VARCHAR,
  external_identity_number VARCHAR
);

CREATE TABLE services (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  organization_id BIGINT references organizations(id) NOT NULL
);

CREATE TABLE people_in_circle (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  circle_id BIGINT NOT NULL references circles(id) ON DELETE CASCADE,
  person_id BIGINT NOT NULL references people(id) ON DELETE CASCADE
);

CREATE TABLE services_in_circle (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  circle_id BIGINT NOT NULL references circles(id) ON DELETE CASCADE,
  service_id BIGINT NOT NULL references people(id) ON DELETE CASCADE
);

-- adds update column trigger for every table
DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT table_name FROM information_schema.tables where table_schema = 'public' 
    LOOP
        EXECUTE format('
          ALTER TABLE %I ADD created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT now() NOT NULL;
          ALTER TABLE %I ADD updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT now() NOT NULL;
          CREATE TRIGGER trigger_update_timestamp
            BEFORE UPDATE ON %I
            FOR EACH ROW EXECUTE PROCEDURE update_modified_column();'
    , t, t, t);
    END loop;
END;
$$ language 'plpgsql';
