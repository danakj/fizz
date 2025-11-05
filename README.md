# FizzBot

FizzBot is a Discord bot for periodic updates on Github PRs and leads issues
waiting for review.

## Architecture

FizzBot communicates with the Discord and Github services, and has no other
user-facing surface.

The application runs as a persistent service that connects to the Discord
service, though which it maintains a presence in the Carbon Discord server
(aka "guild" in the source code).

### User control

From within Discord, users can interact with the bot through text commands,
which are send and received as direct messages between the user and the bot,
but attached to the particular guild from which they are sent. Users can set
their Github username, in order to hear about reviews that are waiting for
them in the Carbon repository. The bot contains rough defaults but allows the
user to customize when notifications will arrive for them, by specifying days
of the week and times at which the notifications should occur. The full list
of preferences a user can set are in the
[`model::UserConfig`](https://github.com/carbon-language/fizz-bot/blob/dcb8f63c24060a5e8604ee9ee4147b9a359de15f/src/model/config.rs#L16-L43)
structure.

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
  structure and stable application-specific Ids for Discord and Github
  users.

* `error.rs` is an impl of `std::error::Error` for application-specific error
  information, outside of the `discord/` module.
