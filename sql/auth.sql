drop table auth;

create table auth (
    username varchar (255),
    pwd_hash varchar (255),
    is_admin char(1) default '0',

    customer_id int,

    primary key (username),
    foreign key (customer_id) references customer
);
