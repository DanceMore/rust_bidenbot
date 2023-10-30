# rust_bidenbot

a silly toy to manage a voting channel for some friends.

## features
* zero config; auto-detects channel name `#biden`
* automatically runs on a Scheduler every Tuesday at 00:02 and 23:58 => https://github.com/DanceMore/rust_bidenbot/blob/main/src/main.rs#L132-L136
** permissions are enforced via extremely minimal `PermissionsOverwrite` directly on the channel
* unfinished `/open` and `/close` commands!

## required permissions

I tried to keep it minimal, but I think it needs both `Manage Role` **and** `Manage Channel`.

it also needs `bot` and `application.command`.
