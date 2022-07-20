# Kagurin.rs's command list

### Help commands
- `kgrs!help`: show command help(hub).
- `kgrs!help display`: show help for display commands.
- `kgrs!help util`: show help for utility commands.
- `kgrs!help fun`: show help for entertainment commands.
- `kgrs!help util`: show help for utility commands.
- `kgrs!help tetr`: show help for TETR.IO related commands.
- `kgrs!help mod`: show help for administrator commands
- `kgrs!help trusted`: show help for commands for people trusted by developer.
- `kgrs!help dev`: show help for commands for Rinrin.

### Display commands
- `kgrs!info`: display Kagurin.rs's information.
- `kgrs!ping`: pong!
- `kgrs!profile [UserID:int]`: display details of the target user.  
if no arguments are passed, display the those of the user who called the command.
- `kgrs!avatar [UserID:int]`: display the target user's icon.  
if no arguments are passed, display the those of the user who called the command.
- `kgrs!server_info [ServerID:int]`: display details of the target server.  
if no arguments are passed,display the those of the server where called the command.
- `kgrs!sky`: display the next daily reset time of Sky:CotL.
- `kgrs!invite`: show invitation URL for this bot.

### Utility commands
- `kgrs!now`: get the current UNIX timestamp.
- `kgrs!timestamp <year:int> <month:int> <day:int> [hour:int] [minute:int] [second:int] [millisecond:int]`: get the UNIX timestamp for the specified date and time.  
- `kgrs!uuid [HowMany:int] [IsUppercase:bool]`: generate UUID(s)

### Entertainment commands
- `kgrs!gtb`: traditional Onion-Garlic-Burrito random

### [TETR.IO](https://tetr.io) related commands
- `kgrs!tetr-user <user:str>`: display details of the target TETR.IO user.

### Administrator commands
- 

### Commands for people trusted by developer

- `kgrs!set_activity <ActivityType:ACTIVITY-TYPE> <content:str>`: chenge Kagurin'rs's activity.  
argument `ACTIVITY-TYPE` is one of `playing`, `listening`, `watching`, `competing`