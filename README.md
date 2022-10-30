# Oversync
## What is this
This is actually my first practical project in rust, just a little bot that pulls feeds and syncs them to platforms like discord. 
## Credits
Icon by feather icons
## Technical Specifacations
### Database
Right now it is file backed but I'm thinking of in the future backing it via MongoDB or SQL as well. Basically we have "collections" which are folders on the filesystem. For this use case each collection is a time interval, so we have `hourly`, `daily`, and `weekly`. Whenever each of these time intervals happens, we go through every item in the collection and perform the apporiate action (usually resyncing a feed with the specified platform). Each feed is keyed (their filename for this database) by the id of the group managing it (for example guild id on discord).

Whenever a service (defined below) wants to access the database, it needs to lock it via the mutex since we have in a `Arc<Mutex<Database>>`. I believe there may be performance implications of doing this such as a user trying to update their configuration when the hourly sync operation enumerating through all the feeds and then the command timing out. 
### Services
Each portion of this system is split into a module that exports a struct implementing the Service trait. Right now the discord bot is the only service in the code but I'm thinking of making an updater that signals other services to provide updates. 
### Discord
A single command exists at the moment called `/configure`, where the user uploads a configuration file in the json format. The file does not nesscarily need to be in json, since we are using `serde` we can also support nicer formats like `yaml` and `toml`. Uploading and deserialzing the format works for now, but due to the way the database is being accessed, the shared `get_database` function is panicking.
```
Command sync done
thread 'tokio-runtime-worker' panicked at 'called `Option::unwrap()` on a `None` value', src/bot/discord/slash_commands/shared.rs:24:54
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
It appears that the database is not properly put into the context's data. 