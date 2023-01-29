# WP Engine CLI v2

A improved version of my earlier tool. This version is build with Rust whereas the previous version was built using JavaScript with node.
I have also opted to not use Async which will speed up performance.

The purpose of this tool is to provide an interface with the wpengine API. I plan on creating a headless
version of the tool to be implemented as part of a pipeline or other similar use case.

## Installation

## Authentication

Running the CLI for the first time will prompt you to add a username and password for the wpengine API.

You can also use the `auth` command to update the credentials or manually add them.

## Sites

The `sites` command will allow you to list, add, update, or delete existing sites.

## Installs

## Accounts

## Users

## Roadmap

- [ ] add bulk edit options.
- [ ] integrate WordPress CLI. e.g: Updating WordPress Sites
- [ ] complete all WP Engine API endpoints
- [ ] implement headless version so the tool can be used in pipelines
