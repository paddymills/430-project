drop trigger set_customer_id;
drop trigger set_loan_id;
drop trigger set_transaction_id;

drop sequence customer_id_counter;
drop sequence loan_id_counter;
drop sequence transaction_id_counter;

drop table mortgage_loan;
drop table auto_loan;
drop table personal_loan;
drop table auth;
drop table transaction;
drop table loan;
drop table customer;


create table customer (
    customer_id int,

    first_name varchar (255),
    last_name varchar (255),
    email varchar(255),
    phone varchar(12),

    primary key (customer_id)
);

create table loan (
    loan_id int,
    customer_id int not null,

    loan_amount float (2),
    interest_rate float (2),
    amount_paid float (2),
    start_date date,
    end_date date,
    number_of_payments int,

    primary key (loan_id),
    foreign key (customer_id) references customer
);

create table transaction (
    transaction_id int,
    customer_id int not null,
    loan_id int not null,

    transaction_amount float (2),
    transaction_date date,

    primary key (transaction_id),
    foreign key (customer_id) references customer,
    foreign key (loan_id) references loan
);

create table auth (
    username varchar (255),
    pwd_hash varchar (255),
    is_admin char(1) default '0',

    customer_id int,

    primary key (username),
    foreign key (customer_id) references customer
);

create table mortgage_loan (
    loan_id int not null,

    address varchar (255),
    area float (3),
    num_bedrooms int,
    num_bathrooms int,
    price float (2),

    primary key (loan_id),
    foreign key (loan_id) references loan
);

create table auto_loan (
    loan_id int not null,

    make varchar (255),
    model varchar (255),
    year int,
    vin varchar (17),

    primary key (loan_id),
    foreign key (loan_id) references loan
);

create table personal_loan (
    loan_id int not null,

    purpose varchar (255),

    primary key (loan_id),
    foreign key (loan_id) references loan
);

-- ----------------------------------------------------------------
-- unique id
-- ----------------------------------------------------------------

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


-- transaction table primary key counter
create sequence transaction_id_counter
    start with 1
    increment by 1;

-- loan table primary key generator
create or replace trigger set_transaction_id
before insert on transaction
referencing new as nrow
for each row

begin
    select transaction_id_counter.nextval
    into :nrow.transaction_id
    from dual;
end;


-- ----------------------------------------------------------------
-- customer ops
-- ----------------------------------------------------------------
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

-- ----------------------------------------------------------------
-- transaction ops
-- ----------------------------------------------------------------
create or replace procedure add_transaction(
    cid transaction.customer_id%type,
    lid transaction.loan_id%type,
    dt transaction.transaction_date%type,
    amt transaction.transaction_amount%type
) as
begin
    declare
        lid_count int;
    begin
        select count(loan_id) into lid_count
        from loan
        where loan_id = lid;

        if lid_count = 0 then
            raise_application_error(-20401, 'No loan exists with specified ID');
        end if;

        insert into transaction (customer_id, loan_id, transaction_date, transaction_amount)
        values (cid, lid, dt, amt);
    end;
end;

create or replace procedure change_transaction(
    tid transaction.transaction_id%type,
    dt transaction.transaction_date%type,
    amt transaction.transaction_amount%type
) as
begin
    declare
        tid_count int;
    begin
        select count(transaction_id) into tid_count
        from transaction
        where transaction_id = tid;

        if tid_count = 0 then
            raise_application_error(-20401, 'No transaction exists with specified ID');
        end if;

        update transaction
        set
            transaction_date = dt,
            transaction_amount = amt
        where
            transaction_id = tid;
    end;
end;

create or replace procedure remove_transaction(
    tid transaction.transaction_id%type
) as
begin
    delete from transaction
    where transaction_id = tid;
end;


-- ----------------------------------------------------------------
-- test data
-- ----------------------------------------------------------------

insert all
    into customer values (null, 'Patrick', 'Miller', 'pjm6196@psu.edu', '717-222-2345')
    into customer values (null, 'Armin', 'van Buuren', 'armin@armind.radio', '222-658-1234')
    into customer values (null, 'Alexander', 'Popov', 'ap123@armind.radio', '513-789-4562')
    into customer values (null, 'Ruben', 'de Ronde', 'rdr3@armind.radio', '717-333-4526')
    into customer values (null, 'Luke', 'Bond', 'bond@armind.radio', '717-458-2379')
select * from dual;
commit;

insert all
    into loan values (null, 1, 75000.0, 0.02, 0.0, '01-JAN-22', '01-JAN-27', 60)
    into loan values (null, 3, 120000.0, 0.04, 100.0, '01-FEB-2022', '01-FEB-2026', 48)
    into loan values (null, 3, 63000.0, 0.10, 10000.0, '01-DEC-21', '01-DEC-26', 60)
    into loan values (null, 2, 500000.0, 0.08, 500.0, '01-JAN-22', '01-JAN-25', 36)
    into loan values (null, 4, 5000.0, 0.04, 30.0, '01-MAR-22', '01-MAR-23', 12)
    into loan values (null, 5, 6730.0, 0.03, 100.0, '12-FEB-22', '12-FEB-23', 12)
    into loan values (null, 4, 23000.0, 0.05, 0.0, '01-JAN-22', '01-JAN-23', 12)
select * from dual;
commit;


insert all
    into auto_loan values (1, 'Audi', 'RS5', 2022, 'VINIS17CHARACTERS')
    into auto_loan values (2, 'Mercedes Benz', 'C63 AMG', 2020, 'ANOTHER117TENCHAR')
    into mortgage_loan values (3, '222 Main St, Lancaster, PA 17601', 1000.000, 3, 2, 60000.0)
    into mortgage_loan values (4, '117 Market St, Elizabethtown, PA 17022', 9000.000, 4, 2, 480000.0)
    into mortgage_loan values (5, '222 Plum St, Somewhere, MD 21999', 300.000, 1, 1, 4850.0)
    into personal_loan values (6, 'an awesome computer')
    into personal_loan values (7, 'because i am broke')
select * from dual;
commit;

insert all
    into auth values ('admin', '3838bd5806d32cd91144865aa822b9551417dd2796c163d390baa7074d3067a7', 1, null)
    into auth values ('cust1', 'be9aa7494d82bfe5fbee0aaf1131f1df313be491e1fecb7aba9dbbf41a0d1bd4', 0, 1)
    into auth values ('cust2', 'adf27cc54e26ca64846f6b05e1b81338e2b1a14fb01b91e7589c73f4da87e3b7', 0, 4)
select * from dual;
commit;
