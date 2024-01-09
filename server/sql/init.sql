CREATE EXTENSION "pgcrypto";

CREATE TABLE email (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    address character varying NOT NULL,
    verified boolean NOT NULL DEFAULT false,
    created timestamp with time zone NOT NULL DEFAULT clock_timestamp(),
    updated timestamp with time zone NOT NULL DEFAULT clock_timestamp(),
    deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT email_pk PRIMARY KEY (id)
);
CREATE UNIQUE INDEX email_address_idx
    ON email USING btree (address);

CREATE TABLE account (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    username character varying UNIQUE NOT NULL,
    password character varying NOT NULL,
    primary_email uuid NOT NULL,
    created timestamp with time zone NOT NULL DEFAULT clock_timestamp(),
    updated timestamp with time zone NOT NULL DEFAULT clock_timestamp(),
    deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT account_pk PRIMARY KEY (id),
    CONSTRAINT account_fk_email FOREIGN KEY (primary_email) REFERENCES email(id)
);
CREATE UNIQUE INDEX account_username_password_idx
    ON account USING btree (username, password);
