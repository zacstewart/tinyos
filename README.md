# TinyOS

A tiny operating system kernel written in Rust and a tiny bit of assembly. I
was inspired to hack on this while reading [The Little Book About OS
Development][little book] during a 26-hour flight.

![screenshot](http://cl.ly/image/2a0a2e0J2s0F/Screen%20Shot%202015-02-01%20at%2011.33.31%20AM.png)

## Build it

I build this inside a Vagrant instance:

```sh
$ vagrant init hashicorp/precise32
$ vagrant up
$ vagratn ssh
$ cd /vagrant
$ make clean all
```

And then run it locally using QEMU:

```sh
$ make run
```

[little book]: http://littleosbook.github.io/
