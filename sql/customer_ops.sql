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
    declare
        cid_count int;
    begin
        select count(customer_id) into cid_count
        from customer
        where customer_id = cid;

        if cid_count = 0 then
            raise_application_error(-20401, 'No customer exists with specified ID');
        end if;

        update customer
        set
            first_name = fname,
            last_name = lname,
            email = new_email,
            phone = new_phone
        where
            customer_id = cid;
    end;
end;

create or replace trigger customer_has_loans
before delete on customer
referencing old as orow
for each row
declare
    cid_count int;
begin
    select count(customer_id) into cid_count
    from customer
    where customer_id = :orow.customer_id;

    if cid_count > 0 then
        raise_application_error(-20999, 'Cannot remove customer with open loans');
    end if;
end;

create or replace procedure remove_customer(
    cid customer.customer_id%type
) as
begin
    delete from customer
    where customer_id = cid;
end;