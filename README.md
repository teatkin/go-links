# Go Links

Go links is a server application that redirects a user to a URL specified by a given shortlink, for instance, `go/gh`
might redirect the user to `https://github.com/myorg`, while `go/bugs` might reidrect the user to 
`https://myorg.internal/bugzilla` or `go/prod-chat` might redirect the user to `ircs://irc.myorg.internal/production`

This is a project that is by its nature designed to be community driven. While there may be a generic set of links for
usual services such as git, team chat servers and so on, there will likely be links teams want to share among themselves
and links an individual may want to create for themselves. The ecosystem of shortlinks becomes better the more people
collaborate and include links to resources they find useful as well.

## Configuration
The service is configured via a TOML file which is usually found in `/etc/golinks/config.toml` but can be specified with
the `-c` flag (TODO)

A sample config file may look as follows
```toml
address = "0.0.0.0"
port = 443

[log]
level = "info"
access = "/var/log/golinks/access.log"
error = "/var/log/golinks/error.log"

[redis]
address = "10.1.2.3"
# port = 6379
```

## Hacking 
This is a standard Rust project so you should be able to get up and running with the above config file and a call to 
`cargo run` to get started.

### TODO 
 - [ ] Add redis functionality
 - [ ] Add dockerfile
 - [ ] Add docker-compose.yml for separate links server and redis container
 - [ ] Add authentication? PKI support?
 - [ ] Mark links as requiring authentication
