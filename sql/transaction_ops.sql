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
