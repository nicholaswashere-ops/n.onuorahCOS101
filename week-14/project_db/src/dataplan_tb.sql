--
-- PostgreSQL database dump
--

\restrict wBryaWatQ9GYrzxrtYgTsMnm29IiAA54LcmNpuzSnn5m3mYtykH1HgwlESMr8zW

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    customer_id integer NOT NULL,
    customer_name character varying(20) NOT NULL,
    customer_age integer NOT NULL,
    customer_email character varying(20) NOT NULL,
    c_phone_no real CONSTRAINT customer_c_phhone_no_not_null NOT NULL,
    employee_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer CONSTRAINT dataplan_table_data_id_not_null NOT NULL,
    data_size character varying(7) CONSTRAINT dataplan_table_data_size_not_null NOT NULL,
    data_duration integer CONSTRAINT dataplan_table_data_duration_not_null NOT NULL,
    data_price integer CONSTRAINT dataplan_table_data_price_not_null NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    department_manager_id integer NOT NULL,
    department_no integer NOT NULL,
    department_name character varying(15) NOT NULL,
    department_location character varying(10) NOT NULL,
    post_no integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    phone_no integer NOT NULL,
    project_name character varying(1) NOT NULL,
    project_duration character varying(20) NOT NULL,
    project_manager_id integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_employee_id_not_null NOT NULL,
    staff_name character varying(20) CONSTRAINT employees_employee_name_not_null NOT NULL,
    department_no integer CONSTRAINT employees_department_no_not_null NOT NULL,
    staff_sal character varying(6) CONSTRAINT employees_employee_salary_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (customer_id, customer_name, customer_age, customer_email, c_phone_no, employee_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	8.055089e+09	102	5
111	Lilian Jaiya	43	I_jaiye@gmail.com	8.0551854e+09	100	3
112	Arthur Musa	50	a_musa@gmail.com	7.0552827e+09	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	9.052357e+09	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	8.0533335e+09	120	5
115	Oghenero Agor	50	o_agor@gmail.com	7.055567e+09	117	11
116	Adams Bree	33	a_bree@gmail.com	8.0567654e+09	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	8.0567634e+09	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	7.0567747e+09	117	11
119	Lawal Tamire	35	I_tamire@gmail.com	9.052111e+09	107	5
120	James Job	44	j_job@gmail.com	8.059694e+09	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	7.0512323e+09	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	8.0549217e+09	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (department_manager_id, department_no, department_name, department_location, post_no) FROM stdin;
108	1	Administration	ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (phone_no, project_name, project_duration, project_manager_id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, department_no, staff_sal, age, mobile) FROM stdin;
100	mustapha ali	3	175000	32	08063285831
107	alokwe Martin	7	380000	48	07090082812
97	Dankade Aminat	5	550000	40	09023688832
108	josiah joshua	1	120000	30	08053189131
102	Mankinde Mary	2	450000	55	09023487830
120	Adeleke jane	4	200000	38	07061045862
122	Osahon mark	6	320000	44	08022289842
117	Suleman Ajayi	3	800000	50	07030089811
104	kuti lawal	1	750000	35	09145689842
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (customer_id);


--
-- Name: dataplan dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (department_manager_id);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (phone_no);


--
-- PostgreSQL database dump complete
--

\unrestrict wBryaWatQ9GYrzxrtYgTsMnm29IiAA54LcmNpuzSnn5m3mYtykH1HgwlESMr8zW

