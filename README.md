# DrawMaster

A Discord bot that splits users in a voice channel into teams or randomly picks a winner — ideal for quick team matchups and fun contests.

---

## Features

- **/teamup** command: Teams up users in a voice channel randomly.
- **/winner** command: Selects a random winner among users in the same voice channel.
- Student-built project for casual and competitive Discord gaming communities.

---

## Usage

Add the bot to your Discord server, then use the following slash commands:

```bash
/teamup
/winner
```

---

## Demo Videos

### `/teamup` Demo

![split into 2 teams](description_resources/split_2teams_reshuffle.mp4)
![split into 3 teams](description_resources/split_3teams_reshuffle.mp4)
![teamup with channel option](description_resources/teamup_with_channels_option.mp4)

### `/winner` Demo

![winner](description_resources/winner.mp4)

---

## Installation

1. **Clone the repository:**

   ```bash
   git clone <repo-url>
   cd Discord-Bot
   ```

2. **Create a `Secrets.toml` file** with the following format:

   ```toml
   DISCORD_TOKEN = "your-bot-token-here"
   GUILD_ID = "your-discord-guild-id"
   ```

3. **Build and run the project:**

   Using Shuttle:
   ```bash
   cargo shuttle run --release
   ```

   Or using Docker:
   ```bash
   docker build -t drawmaster .
   docker run -d --name drawmaster drawmaster
   ```

---

## Dependencies

Dependencies are managed via `Cargo.toml` and locked in `Cargo.lock`.

---

## License

This project uses the MIT license.

---

## Authors

- WSm-77
- NCCMNT

---

## Target Audience

- Gamers using Discord voice channels.


