use mysql::*;
use mysql::prelude::*;
use actix_web::{get, web, App, HttpServer, Responder};
use blog_back::db::migration::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name:  Option<String>,
}

fn create(i: i32, a: i32, n: Option<String>) -> Payment {
    Payment {
        customer_id: i,
        amount: a,
        account_name: n,
    }
}
#[actix_web::main]
async fn main() -> Result<()> {
    // db migration
    let pool = Pool::new("mysql://root:example@127.0.0.1:3306/blog")?;
    let mut conn = pool.get_conn();
    if let Ok(mut c) = conn {
        println!("good, {:?}", c);
        let r = embed::migrations::runner().run(&mut c);
        if let Err(e) = r {
            println!("{:?}", e);
        }
    } else {
        panic!("{:?}", conn);
    }

    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::get().to(index)))
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await?;
    Ok(())
}

async fn index() -> impl Responder {
    format!("Hello, World, Hello, Howard")
}
// fn main()->Result<()> {
//     println!("Hello, world!");
//     let pool = Pool::new("mysql://root:example@127.0.0.1:3306/blog")?;
//     let mut conn = pool.get_conn()?;
//
//     let payments = vec![
//         create(1, 2, None),
//         create(3, 4, Some(String::from("aoo")))
//     ];
//     conn.exec_batch(r"INSERT INTO payment (customer_id, amount, account_name) VALUES (:customer_id, :amount, :account_name)",
//                     payments.iter().map(|p| params! {
//                     "customer_id" => p.customer_id,
//                     "amount" => p.amount,
//                     "account_name" => &p.account_name,
//                     })
//     )?;
//
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(true, true);
    }
}