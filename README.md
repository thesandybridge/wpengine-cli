# WP Engine CLI v2

An improved version of my earlier tool. This version is built with Rust whereas the previous version was built using JavaScript with node.
I have also opted to not use async which will speed up performance.

The purpose of this tool is to provide an interface with the wpengine API. I plan on creating a headless
version of the tool to be implemented as part of a pipeline or other similar use case.

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

- [ ] Add bulk edit options.
- [ ] Integrate WordPress CLI. e.g: Updating WordPress Sites
- [ ] https://github.com/thesandybridge/wpengine_cli_v2/issues/7
- [ ] https://github.com/thesandybridge/wpengine_cli_v2/issues/8
- [ ] https://github.com/thesandybridge/wpengine_cli_v2/issues/4
- [ ] https://github.com/thesandybridge/wpengine_cli_v2/issues/5
- [x] Implement headless version so that the tool can be used in pipelines
