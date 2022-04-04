
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