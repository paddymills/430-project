create or replace trigger set_customer_id
before insert on customer
for each row

declare
    next_id customer.customer_id%type := 0;
begin
    select max(customer_id) + 1
    into :next_id
    from customer;

    set :new.customer_id = :next_id;
end;

insert into customer(first_name, last_name) values ('a', 'b');
insert into customer(first_name, last_name) values ('c', 'd');
commit;

select * from customer;
