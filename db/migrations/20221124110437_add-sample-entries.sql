-- migrate:up
insert into todolist (id, title, date)
values (1, 'nothing', 1669288106),
       (2, 'something', 1669288164),
       (3, 'coding', 1669288664),
       (4, 'sleeping', 1669288964);

-- migrate:down
delete
from todolist
where title in ('nothing', 'something', 'coding', 'sleeping');
