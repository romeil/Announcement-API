--
-- PostgreSQL database dump
--

-- Dumped from database version 16.2
-- Dumped by pg_dump version 16.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: random_between(integer, integer); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE OR REPLACE FUNCTION public.random_between(low integer, high integer) RETURNS integer
    LANGUAGE plpgsql STRICT
    AS $$
BEGIN
RETURN floor(random()* (high-low+1) + low);
END;
$$;


ALTER FUNCTION public.random_between(low integer, high integer) OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: announcement; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.announcement (
    announcement_uid uuid NOT NULL,
    info character varying(400) NOT NULL,
    date character varying(10) NOT NULL,
    club_uid uuid NOT NULL
);


ALTER TABLE public.announcement OWNER TO postgres;

--
-- Name: club; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.club (
    club_uid uuid NOT NULL,
    name character varying(100) NOT NULL,
    password_hash character varying(100) NOT NULL,
    email character varying(100) NOT NULL
);


ALTER TABLE public.club OWNER TO postgres;

--
-- Name: pending_users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.pending_users (
    user_uid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    email character varying(100) NOT NULL,
    role character varying(20) NOT NULL,
    registration_id character varying(9) NOT NULL,
    temporary_pin character varying(9),
    password_hash character varying(100)
);


ALTER TABLE public.pending_users OWNER TO postgres;

--
-- Name: prefect; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.prefect (
    prefect_uid uuid NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    email character varying(100) NOT NULL,
    password_hash character varying(100) NOT NULL
);


ALTER TABLE public.prefect OWNER TO postgres;

--
-- Name: announcement announcement_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'announcement_pkey'
    ) THEN
        ALTER TABLE ONLY public.announcement
        ADD CONSTRAINT announcement_pkey PRIMARY KEY (announcement_uid);
    END IF;
END $$;

--
-- Name: club club_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'club_name_key'  
    ) THEN
        ALTER TABLE ONLY public.club
        ADD CONSTRAINT club_name_key UNIQUE (name);
    END IF;
END $$;

--
-- Name: club club_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'club_pkey'  
    ) THEN
        ALTER TABLE ONLY public.club
        ADD CONSTRAINT club_pkey PRIMARY KEY (club_uid);
    END IF;
END $$;

--
-- Name: pending_users pending_users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'pending_users_pkey'  
    ) THEN
        ALTER TABLE ONLY public.pending_users
        ADD CONSTRAINT pending_users_pkey PRIMARY KEY (user_uid);
    END IF;
END $$;


--
-- Name: pending_users pending_users_registration_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'pending_users_registration_id_key'  
    ) THEN
        ALTER TABLE ONLY public.pending_users
        ADD CONSTRAINT pending_users_registration_id_key UNIQUE (registration_id);
    END IF;
END $$;

--
-- Name: prefect prefect_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'prefect_pkey'  
    ) THEN
        ALTER TABLE ONLY public.prefect
        ADD CONSTRAINT prefect_pkey PRIMARY KEY (prefect_uid);
    END IF;
END $$;


--
-- Name: announcement announcement_club_uid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'announcement_club_uid_fkey'
    ) THEN
        ALTER TABLE ONLY public.announcement
        ADD CONSTRAINT announcement_club_uid_fkey FOREIGN KEY (club_uid) REFERENCES public.club(club_uid);
    END IF;
END $$;

--
-- PostgreSQL database dump complete
--

