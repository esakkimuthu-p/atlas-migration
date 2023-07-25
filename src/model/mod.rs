use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

mod acc_trn;
mod batch;
mod inv_trn;
mod inventory;

pub use acc_trn::AccountTransaction;
pub use batch::Batch;
pub use inv_trn::InventoryTransaction;
pub use inventory::*;

// fn serialize_opt_id_as_thing<S: Serializer, const KEY: char>(
//     val: &Option<ObjectId>,
//     ser: S,
// ) -> Result<S::Ok, S::Error> {
//     let key = match KEY {
//         'I' => "inventory",
//         _ => "err",
//     };
//     match val {
//         Some(id) => Thing::from((key, id.to_hex().as_str())).serialize(ser),
//         _ => unreachable!(),
//     }
// }

// fn serialize_id_as_thing<S: Serializer, const KEY: char>(
//     val: &ObjectId,
//     ser: S,
// ) -> Result<S::Ok, S::Error> {
//     let key = match KEY {
//         'U' => "unit",
//         _ => "err_binding",
//     };
//     Thing::from((key, val.to_hex().as_str())).serialize(ser)
// }
