Patrick Miller
CMPSC 430-002
Final Project (Project A)

Before running
    1) run `schema.sql` to load schema, triggers, stored procedures and test data into database
    2) edit `db.toml` and set your username and password

There are 3 existing user accounts:
    Admin account (can edit customers and loans)
        username: admin
        password: pwd123
    Customer 1 (can only view their loans, customer_id: 1)
        username: cust1
        password: loans
    Customer 2 (can only view their loans, customer_id: 4)
        username: cust2
        password: mortgage

All the code is compiled on my github at https://github.com/paddymills/430-project
There are aspects of the code that I'm not exactly happy with and would refactor, so consider yourself warned.
