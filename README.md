# TinyOS

A tiny operating system kernel written in Rust and a tiny bit of assembly.

![screenshot](http://cl.ly/image/2f0O142u092F/Screen%20Shot%202015-01-28%20at%2010.30.55%20PM.png)

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
