# `inloc` - An InterNet LOCator tool

Hello! This is a tool that you can use to not only perform DNS requests on a URL, but then you get to see a lot of information derived from that IP address.
This information includes real-world location data, isp info, and an organization name (if applicable).
It also has *colors* 🎨!

## Usage

The tool is very simple to use:

```bash
$ cargo build
...
$ cargo run -- -u <URL> -i <IP>
```

To scan multiple locations, you can simply repeat the flag with a new value. Either tag (and its value) can be omitted with no problems.

For example, If I wanted to perform a DNS lookup on `example.com`:

```bash
$ cargo run -- -u example.com

==== example.com ====
found ip: 93.184.215.14:
	continent: Europe
	country: GB (United Kingdom)
	region/state: England
	city: London
	isp: Edgecast Inc.
```

## Installation

Installing the exectuable to path is a breeze! Just navigate to the project folder, then run this command:

```bash
$ cargo install --path .
```