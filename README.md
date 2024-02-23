# helferbiene-rs

A Rust rewrite of [Helferbiene](https://github.com/ryodari/Helferbiene).

## Features

### Bot Activity Status

Display online players count based on the hostname/ip address provided in the `.env` file.

![Screenshot of discord set bot activity status](https://i.imgur.com/IV8iYMv.png)

### Serverinfo command

Display basic server info based on the hostname/ip address provided as command arguments. 

Note: The port argument is optional. If not provided the standard port `25565` will be used.

#### Usage:

- `/Serverinfo` `[hostname | ip address]` `[optional: port]`
- `/Serverinfo` `[hostname | ip address]`:`[optional: port]`

## Credits

Special thanks to [0x280](https://github.com/0x280) who did the Rust implementation of the [OG Helferbiene](https://github.com/ryodari/Helferbiene) plus the additional `serverinfo` command. >:3