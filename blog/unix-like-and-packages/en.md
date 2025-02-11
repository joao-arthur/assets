# UNIX-Like And Packages

Why sometimes everything seems messy on Linux? why the whole free software community still uses C?

## Make Way For Linux

Generalizing, when someone begins on Linux, it happens with a famous distribution. After install, it
is possible to install the programs using terminal commands. Usually, these commands are just
**CLIs** that behave as **frontend** for the **package managers**.

| distro | frontend | backend |
| ------ | -------- | ------- |
| debian | apt      | dpkg    |
| arch   | pacman   | pacman  |
| fedora | yum      | rpm     |
| fedora | dnf      | rpm     |
| gentoo | emerge   | portage |

> Most common **terminal commands** and **package managers** in Linux world

One of the **free software** principles is the _freedom of choise_. When it comes to package
managers, each distro uses a specific one.

## C, The Lingua Franca

The basic programs of every operational system are all written in **C**. This is not just the reflex
of a 50 years legaly, but also something intentional, because a unique feature of this programming
language: **dynamic linking**. This features allow to compile a program that depends on many
libraries without having them avaliable at compile time, just with the **headers** files.

Imagine for example that in your Linux, 50 programs use a cryptography library called
**libre_crypto**. When running the software update through the package manager, this lib gets
updated from version **2.11** up to **2.12**. All the 50 programs will use this updated lib, with
the bonus of the fixed bugs and the onus of possible new bugs.

In practice, it is a bit more complicated, since every package manager has a different strategy of
packaging to minimize breaking changes with updates. For deepening, I recommend the reading of the
excelent article
[symlinks and .so files on linux - what you need to know](https://dmerej.info/blog/post/symlinks-and-so-files-on-linux/).

### Analogy

It is possible to compare this behavior with **NodeJS**. Imagine the following import:

```js
import React from "react";
```

When the program gets executed, **NodeJS** will lookup for the directory ***node_modules/react***.

Similar way, when a **C** program makes the following include:

```c
#include <SDL2/SDL.h>
```

When the program gets executed, the **Operational System** will lookup for the file
**/usr/lib/libSDL2.so**.

## The Father Of All

Let's briefly return to the _beginning of History, january 1st 1970_. A new Operational System was
being created at Bell Labs, the **UNIX**. In short period of time, this system becamed used in
universities all along the United States and soon people started creating their own **UNIX**, with
many distributions created. As a result, currently, the main open source **UNIX-like** Operational
Systems are:

- GNU/Linux
- OpenBSD
- FreeBSD
- Minix

All these systems have **more or less** the same **APIs** avalialble for any program. What means
that, any program written thinking on **GNU/Linux** can be ported to **OpenBSD** or to **FreeBSD**,
or vice-versa.

This implies that it is possible to run **ZSH**, **fastfetch**, and even **gnome** both on
**GNU/Linux** and **OpenBSD**, and these are _real examples I found on the Internet_.

## Conclusion

The freedom of free software sometimes can lead to compatibility problems and bugs. But it is
important to understand the challenges and advatanges of keeping a portability of programs between
UNIX-like systems.
