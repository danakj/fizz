# Fizz

Fizz is a Discord bot for periodic updates on Github PRs waiting for your
review.

## Code structure

* `discord/` contains the integration with the discord servers. It is built out
  of async functions on top of tokio.
  * `discord/commands/` contains the slash commands that the bot responds to.
    There is one file for each command.
  * `discord/tasks/` contains background tasks that the bot runs continuously.
    There is one file for each task.
  * `discord/util/` contains async helpers for dealing with discord or the model
    from discord tasks and commands.
  * `DiscordData` is the data type passed into all Discord event handlers, and
    contains access to the model.
  * `DiscordError` is the error type for Discord event handlers, and includes
    the ability to include a reply to the user in the error if one should be
    sent back to them, in addition to logging the error. It supports conversion
    from other error types.

* `github/` contains the integration with Github. It is built out of async
  functions on top of tokio.

* `model/` contains the data model of the bot, which includes the Config
  structure and stable application-specific Ids for discord and github
  users.

* `error.rs` is an impl of `std::error::Error` for application-specific error
  information, outside of the `discord/` module.
