
-- customer table primary key counter
create sequence customer_id_counter
    start with 1
    increment by 1;

-- customer table primary key generator
create or replace trigger set_customer_id
before insert on customer
referencing new as nrow
for each row

begin
    select customer_id_counter.nextval
    into :nrow.customer_id
    from dual;
end;


-- loan table primary key counter
create sequence loan_id_counter
    start with 1
    increment by 1;

-- loan table primary key generator
create or replace trigger set_loan_id
before insert on loan
referencing new as nrow
for each row

begin
    select loan_id_counter.nextval
    into :nrow.loan_id
    from dual;
end;
