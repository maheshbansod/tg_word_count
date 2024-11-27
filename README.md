
## word_count

script to download telegram messages and search a word written in different combinations in those
messages, and count the number of words.

See tests in [`src/lib.rs`](./src/lib.rs) for examples of combinations of the word that will be accepted.

The `TG_ID` and `TG_HASH` environment variables must be set (learn how to do it for
[Windows](https://ss64.com/nt/set.html) or [Linux](https://ss64.com/bash/export.html))
to Telegram's API ID and API hash respectively.
