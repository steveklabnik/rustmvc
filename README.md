# rustmvc

[![Build Status](https://travis-ci.org/steveklabnik/rustmvc.svg)](https://travis-ci.org/steveklabnik/rustmvc)

This is a [TodoMVC](http://todomvc.com/) implementation, using
[Ember](http://emberjs.com/) on the front end and
[Nickel.rs](http://nickel.rs/) on the back end.

In other words: Rust + Ember, sitting in a tree. K I S...

## Getting started

You'll need a postgres instance running on localhost, with a
`rustmvc` user and a `rustmvc` database:

```bash
$ createrole rustmvc
$ createdb rustmvc
```

When you've got that going...

```bash
$ git clone https://github.com/steveklabnik/rustmvc
$ cd rustmvc
$ ./target/create_databases # you only need this the first time
$ cargo build
$ ./target/rustmvc
$ firefox http://localhost:6767/ # in a different shell, of course
$ curl --data "{\"title\":\"Title ha ha ha\",\"is_completed\":false}" http://localhost:6767/todos
```
