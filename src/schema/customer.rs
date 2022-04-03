
use oracle::{self, Connection, Row, RowValue};
use tabled::Tabled;

#[derive(Tabled)]
pub struct Customer {
    pub customer_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String
}

impl RowValue for Customer {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            customer_id: row.get("customer_id")?,
            first_name: row.get("first_name")?,
            last_name: row.get("last_name")?,
            email: row.get("email")?,
            phone: row.get("phone")?
        })
    }
}

pub trait CustomerOps {
    fn add_customer(self: Self, fname: &String, lname: &String, email: &String, phone: &String) -> oracle::Result<()> where Self: Sized;
    fn edit_customer(self: Self, id: &u32, fname: &String, lname: &String, email: &String, phone: &String) -> oracle::Result<()> where Self: Sized;
    fn remove_customer(self: Self, id: &u32) -> oracle::Result<()> where Self: Sized;
    fn find_customer(self: Self, fname: &String, lname: &String) -> Option<Vec<Customer>> where Self: Sized;
    fn list_customers(self: Self) -> Option<Vec<Customer>> where Self: Sized;
}

impl CustomerOps for Connection {
    fn add_customer(
        self: Connection,
        fname: &String,
        lname: &String,
        email: &String,
        phone: &String
    ) -> oracle::Result<()> {
        let _ = self.execute(
            "call add_customer(:1, :2, :3, :4)",
            &[fname, lname, email, phone]
        );
        
        self.commit()
    }

    fn edit_customer(
        self: Self,
        id: &u32,
        fname: &String,
        lname: &String,
        email: &String,
        phone: &String
    ) -> oracle::Result<()> {
        let _ = self.execute(
            "call change_customer(:1, :2, :3, :4, :5)",
            &[id, fname, lname, email, phone]
        );
        
        self.commit()
    }

    fn remove_customer(self: Self, id: &u32) -> oracle::Result<()> {
        let _ = self.execute(
            "call remove_customer(:1)",
            &[id]
        );
        
        self.commit()
    }
    
    fn find_customer(self: Self, fname: &String, lname: &String) -> Option<Vec<Customer>> {
        let res = self.query_as::<Customer>(
            "
                select *
                from customer
                where first_name = :1 and last_name = :2
            ", &[fname, lname]
        );

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn list_customers(self: Self) -> Option<Vec<Customer>>
        where Self: Sized
    {
        let res = self.query_as::<Customer>("select * from customer", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

}
