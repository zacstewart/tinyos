# TinyOS

A tiny operating system kernel written in Rust and a tiny bit of assembly. I
was inspired to hack on this while reading [The Little Book About OS
Development][little book] during a 26-hour flight.

![screenshot](http://cl.ly/image/2a0a2e0J2s0F/Screen%20Shot%202015-02-01%20at%2011.33.31%20AM.png)

## Features

* Frame buffer (screen) text output
* Serial port text output (for debugging)

## Build Requirements

I'm currently only building this on Mac OS X, but previously I built it in a
Vagrant instance running Ubuntu. Using the `ld` provided by the system, you
should be able to as well.  Just install the following packages using `apt` or
your package manager, and update the _Makefile_ to point to your `ld`.

* Cdrtools (`brew install cdrtools`)
* NASM (`brew install nasm`)
* Qemu (`brew install qeumu`)
* GNU Binutils built for i386 ELF format

        $ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils.tar.bz2'
        $ ./configure --target=i386-elf --program-prefix=i386-elf-
        $ make && make install

# Build

```sh
$ make clean all
```

And then run it locally using QEMU:

```sh
$ make run
```

[little book]: http://littleosbook.github.io/
[binutils]: http://www.gnu.org/software/binutils/
