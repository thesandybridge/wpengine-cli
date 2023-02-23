# WP Engine CLI

Built with Rust, this tool will allow you to control your sites, installs, users and more from your terminal or within a pipeline using headless mode.
I have also opted to not use async which will speed up performance. I work on this in my spare time, features may be added and removed randomly until
I push a version **1.0.0** release.

[wpengine API reference](https://wpengineapi.com/reference)

> **Warning**
>
> This project is not constantly worked on and is unstable. Use at your own risk. As long as the version is below **1.0.0** you may experience breaking changes.

## Installation

You will need Rust installed in order to install this from Crates.io, however, on github there are binaries for Windows, Mac, and Linux that you can install.
Those will not be automatically updated (right now). So you will need to manually install the new binary whenever I release a update.

## Authentication

Running the CLI for the first time will prompt you to add a username and password for the wpengine API.

You can also use the `auth` command to update the credentials or manually add them.

## Sites

The `sites` command will allow you to list, add, update, or delete existing sites. Passing the 
`-H` flag will enable headless mode for using the CLI in a pipeline or as part of a script.

Headless mode will enable list, add, update, and delete sub commands. 

**Examples**

```bash
wpe -H sites add <NAME> <Account_ID> # adds a site with the name provided.
wpe -H sites list # lists all sites for page 1.
wpe -H sites 1 list # lists all sites for page 2.
wpe -H sites list <Site_ID> # list a single site from page 1.
wpe -H sites 1 list <Site_ID> # list a single site from page 2.
```


## Installs

## Accounts

## Users

## Roadmap

### In Progress

- [ ] [Build commands for User/Accounts endpoint](https://github.com/thesandybridge/wpengine_cli_v2/issues/5)
- [ ] [Build commands for Domain endpoint](https://github.com/thesandybridge/wpengine_cli_v2/issues/8)
- [ ] [Build commands for Installs endpoint](https://github.com/thesandybridge/wpengine_cli_v2/issues/4)
- [ ] [Build commands for SSH endpoint](https://github.com/thesandybridge/wpengine_cli_v2/issues/6)

### Optional Features

- [ ] Integrate WordPress CLI. e.g: Updating WordPress Sites
- [ ] Add bulk edit options.

### Completed

- [x] Implement headless version so that the tool can be used in pipelines
