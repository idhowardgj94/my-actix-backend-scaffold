use mysql::{PooledConn, Transaction};

pub enum MySqLDatabase<'a, 't> {
    Transaction(&'a mut Transaction<'t>),
    Connection(&'a mut PooledConn)
}