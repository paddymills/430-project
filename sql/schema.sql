drop trigger set_customer_id;

drop table mortgage_loan;
drop table auto_loan;
drop table personal_loan;
drop table loan;
drop table customer;

create table customer (
    customer_id int,

    first_name varchar (255),
    last_name varchar (255),

    primary key (customer_id) initially deferred
);

create table loan (
    loan_id int not null,
    customer_id int not null,

    loan_amount float (2),
    interest_rate float (2),
    amount_paid float (2),
    start_date date,
    end_date date,
    number_of_payments int,

    primary key (loan_id),
    foreign key (customer_id) references customer initially deferred
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
