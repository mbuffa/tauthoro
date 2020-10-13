--
-- PostgreSQL database dump
--

-- Dumped from database version 13.0
-- Dumped by pg_dump version 13.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: find_user_and_token(character varying); Type: FUNCTION; Schema: public; Owner: tauthoro
--

CREATE FUNCTION public.find_user_and_token(_email character varying) RETURNS TABLE(id integer, encrypted_password character varying, token character varying)
    LANGUAGE sql STABLE
    AS $$
	SELECT u.id, u.encrypted_password, t1.value as "token"
	FROM users u
	JOIN tokens t1 ON u.id = t1.user_id
	LEFT OUTER JOIN tokens t2 ON (u.id = t2.user_id AND t1.id > t2.id)
	WHERE u.email = _email AND t1.revoked_at IS NULL
    LIMIT 1;
$$;


ALTER FUNCTION public.find_user_and_token(_email character varying) OWNER TO tauthoro;

--
-- Name: tokens_id_seq; Type: SEQUENCE; Schema: public; Owner: tauthoro
--

CREATE SEQUENCE public.tokens_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.tokens_id_seq OWNER TO tauthoro;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: tokens; Type: TABLE; Schema: public; Owner: tauthoro
--

CREATE TABLE public.tokens (
    id integer DEFAULT nextval('public.tokens_id_seq'::regclass) NOT NULL,
    user_id integer NOT NULL,
    value character varying,
    token_type character varying,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    revoked_at timestamp with time zone
);


ALTER TABLE public.tokens OWNER TO tauthoro;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: tauthoro
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO tauthoro;

--
-- Name: users; Type: TABLE; Schema: public; Owner: tauthoro
--

CREATE TABLE public.users (
    id integer DEFAULT nextval('public.users_id_seq'::regclass) NOT NULL,
    email character varying,
    encrypted_password character varying,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.users OWNER TO tauthoro;

--
-- Data for Name: tokens; Type: TABLE DATA; Schema: public; Owner: tauthoro
--

COPY public.tokens (id, user_id, value, token_type, created_at, updated_at, revoked_at) FROM stdin;
1	1	yolo	authentication	2020-10-13 15:51:14.545199+00	2020-10-13 15:51:14.545199+00	\N
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: tauthoro
--

COPY public.users (id, email, encrypted_password, created_at, updated_at) FROM stdin;
1	mbuffa@gmail.com	$2y$12$pax0m7e4PuAY7MGNUiZhB.MBqo6QOnRSVkciurYG05eKaI0Qx5fWi	2020-10-13 15:24:49.211369+00	2020-10-13 15:24:49.211369+00
\.


--
-- Name: tokens_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tauthoro
--

SELECT pg_catalog.setval('public.tokens_id_seq', 1, false);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tauthoro
--

SELECT pg_catalog.setval('public.users_id_seq', 1, false);


--
-- Name: tokens tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: tauthoro
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: tauthoro
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: tokens tokens_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tauthoro
--

ALTER TABLE ONLY public.tokens
    ADD CONSTRAINT tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

