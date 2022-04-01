create or replace procedure add_customer(
    fname customer.first_name%type,
    lname customer.last_name%type,
    email_addr customer.email%type,
    phone_num customer.phone%type
) as
begin
    insert into customer (first_name, last_name, email, phone)
    values (fname, lname, email_addr, phone_num);
end;

create or replace procedure change_customer(
    cid customer.customer_id%type,
    fname customer.first_name%type,
    lname customer.last_name%type,
    new_email customer.last_name%type,
    new_phone customer.last_name%type
) as
begin
    update customer
    set
        first_name = fname,
        last_name = lname,
        email = new_email,
        phone = new_phone
    where
        customer_id := cid;
end;

create or replace procedure remove_customer(
    cid customer.customer_id%type
) as
begin
    delete from customer
    where customer_id := cid;
end;

create or replace procedure find_customer(
    fname customer.first_name%type,
    lname customer.last_name%type
) as
begin
    select * from customer
    where first_name := fname
    and last_name := lname;
end;

create or replace procedure get_customer_list
as
begin
    select * from customer;
end;


