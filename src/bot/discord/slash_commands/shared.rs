use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::Command;
use crate::core::service::Service;
use crate::core::db::Database;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct DatabaseInTypeMap;

impl TypeMapKey for DatabaseInTypeMap {
    type Value = Arc<Mutex<Database>>;
}

// TODO: fix
pub async fn get_database(ctx: &Context) -> Arc<Mutex<Database>>{
    let type_map = ctx.data.read().await;
    let db_arc = type_map.get::<DatabaseInTypeMap>().unwrap().clone();
    db_arc
}