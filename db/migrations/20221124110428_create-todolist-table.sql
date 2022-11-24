-- migrate:up
create table todolist (
  id int PRIMARY KEY,
  title varchar(255)  NOT NULL,
  date int NOT NULL
);

-- migrate:down
drop table todolist;
