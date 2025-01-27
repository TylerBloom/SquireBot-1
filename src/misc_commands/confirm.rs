use crate::model::confirmations::confirmation_map::ConfirmationsContainer;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command("yes")]
#[aliases("y")]
#[help_available(false)]
#[description("Confirms your waiting request.")]
async fn yes(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let todos = data.get::<ConfirmationsContainer>().unwrap();
    if let Some((_,mut task)) = todos.remove(&msg.author.id) {
        let response = task.execute();
        msg.reply(&ctx.http, response).await?;
    } else {
        msg.reply(&ctx.http, "Its seems that you don't have anything waiting for your approval.").await?;
    }
    Ok(())
}

#[command("no")]
#[aliases("n")]
#[help_available(false)]
#[description("Denies your waiting request.")]
async fn no(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let todos = data.get::<ConfirmationsContainer>().unwrap();
    if let Some((_,task)) = todos.remove(&msg.author.id) {
        msg.reply(&ctx.http, "Alright, I won't do that.").await?;
    } else {
        msg.reply(&ctx.http, "Its seems that you don't have anything waiting for your approval.").await?;
    }
    Ok(())
}
