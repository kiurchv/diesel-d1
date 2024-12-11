use std::cell::Cell;

use async_trait::async_trait;
use diesel::{connection::TransactionManagerStatus, result::Error as DieselError, QueryResult};
use diesel_async::{AsyncConnection, TransactionManager};
use js_sys::Array;
use wasm_bindgen_futures::JsFuture;

use crate::{binding::D1Result, utils::SendableFuture, D1Connection};


#[derive(Default)]
/// D1 doesn't have transactions but we can emulate a depth=1 transaction using `batch()`
/// FIXME: transactions not fully working atm
pub struct D1TransactionManager{
    pub(crate) is_in_transaction: Cell<bool>,
    status: TransactionManagerStatus
}

#[async_trait]
impl TransactionManager<D1Connection> for D1TransactionManager {
    type TransactionStateData = Self;
    
    async fn begin_transaction(conn: &mut D1Connection) -> QueryResult<()> {
        conn.transaction_state().is_in_transaction.set(true);

        Ok(())
    }
    
    async fn rollback_transaction(conn: &mut D1Connection) -> QueryResult<()> {
        conn.transaction_state().is_in_transaction.set(false);
        conn.transaction_queries.clear();
        Ok(())

    }
    
    async fn commit_transaction(conn: &mut D1Connection) -> QueryResult<()> {
        match conn.transaction_state().is_in_transaction.get() {
            true => {
                if conn.transaction_queries.is_empty() {
                    conn.transaction_manager.is_in_transaction.set(false);
                    conn.transaction_queries.clear();
                    return  Ok(())
                }

                // FIXME: i think that it will never reach this state but okay for now
                let array = conn.transaction_queries.iter()
                    .collect::<Array>();

                let promise = match conn.binding.batch(array) {
                    Ok(res) => res,
                    Err(_) => todo!(),
                };

                let result: D1Result = match SendableFuture(JsFuture::from(promise)).await {
                    Ok(res) => res.into(),
                    Err(_) => todo!(),
                };


                conn.transaction_manager.is_in_transaction.set(false);
                conn.transaction_queries.clear();
                Ok(())
            },
            false => {
                Err(DieselError::NotInTransaction)
            },
        }
    }
    
    #[doc = " Fetch the current transaction status as mutable"]
    #[doc = ""]
    #[doc = " Used to ensure that `begin_test_transaction` is not called when already"]
    #[doc = " inside of a transaction, and that operations are not run in a `InError`"]
    #[doc = " transaction manager."]
    fn transaction_manager_status_mut(conn: &mut D1Connection) ->  &mut TransactionManagerStatus {
        let state = conn.transaction_state();

        &mut state.status
    }

}