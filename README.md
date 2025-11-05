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
[`model::UserConfig`](https://github.com/carbon-language/fizz-bot/blob/dcb8f63c24060a5e8604ee9ee4147b9a359de15f/src/model/config.rs#L15-L43)
structure.

All commands are grouped together under `/fizz` so a command looks like
`/fizz command_name args...`. The commands available to users are found in
[`src/discord/commands/`](/src/discord/commands). Users access these by
typing `/fizz` in a channel of the Carbon Discord server, at which point
the Discord UI provides autocomplete for commands and arguments along with
descriptions of them.

Commands to set preferences for the user are of the form `my_something_is` or
`my_somethings_are`. A user can query all of their preferences with `whoami`
or ask for help with `help`. And a user can remove all of their information
from the bot with the `remove_me` command.

There are a few additional commands for administrators only. Primarily, the
`setup` command which points the bot to the Github repository and the Discord
channel where it should send notification messages. There are a few others
like `whoiseveryone` and `report_all` to test or query the overall status of
the bot.

### Persistence

Whenever a change is made to a user's configuration, including with
`remove_me`, those changes are persisted so that they remain after a restart.
The bot uses a simple TOML file on disk, serialized and deserialized with
[serde](http://serde.rs/).

### Notifications

The bot's function is to periodically wake up, poll the specified Github
repository, collect PRs and leads issues and then send notifications to a
specified channel in the Carbon Discord server.

Notifications are targetted at a user by putting an `@username` in the first
line of the message, so that they are pinged by Discord. The bot [deletes any
past notification messages](
https://github.com/carbon-language/fizz-bot/blob/dcb8f63c24060a5e8604ee9ee4147b9a359de15f/src/discord/tasks/watch_github.rs#L293-L303)
that it had sent to that user before posting the new message, so each user has
at most one copy of a notification message in the channel at a time. If they
have nothing to be notified of, then they will be left with no messages
directed to them.

Pull requests waiting for a review are included in the notification message.
And if the user identifies themself as a "lead", then leads issues marked with the
["blocking work"](https://github.com/carbon-language/carbon-lang/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22blocking%20work%22)
are also included.

At most once a week, leads issues that are not labelled as
["long term issue"](https://github.com/carbon-language/carbon-lang/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22long%20term%20issue%22)
are get sent in separate a notification message, so that it will not be deleted
by the next nofitication of PR reviews.

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
