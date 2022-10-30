use std::sync::Arc;
use std::sync::Mutex;

use crate::core::db::Database;








use serenity::prelude::*;

// used in discord
pub struct DatabaseInTypeMap;

impl TypeMapKey for DatabaseInTypeMap {
    type Value = Arc<Mutex<Database>>;
}

// TODO: fix
pub async fn get_database(ctx: &Context) -> Arc<Mutex<Database>> {
    let type_map = ctx.data.read().await;
    let db_arc = type_map.get::<DatabaseInTypeMap>().unwrap().clone();
    db_arc
}
