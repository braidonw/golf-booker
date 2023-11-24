-- Add migration script here
create table if not exists events (
  id integer primary key,
  active integer,
  date text,
  name text,
  event_date text,
  event_status_code integer,
  event_status_code_friendly text,
  title text not null,
  is_lottery integer,
  can_open_event integer,
  has_competition integer,
  event_type_code integer,
  event_category_code integer,
  event_time_code_friendly text,
  availability integer,
  is_open integer,
  is_ballot integer,
  is_ballot_open integer,
  is_results integer,
  is_matchplay integer
);

