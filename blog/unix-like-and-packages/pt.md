# UNIX-Like E Pacotes

Por que as vezes tudo parece bagunçado no Linux? Por que toda a comunidade de software livre ainda
usa C?

## Abram Alas Para O Linux

Generalizando, quando alguém começa no **Linux** é com uma distribuição famosa. Após a instalação, é
possível instalar os programas a partir de comandos no terminal. Normalmente, esses comandos são
apenas **CLIs** que agem de **frontend** para os **gerenciadores de pacote**.

| distro | frontend | backend |
| ------ | -------- | ------- |
| debian | apt      | dpkg    |
| arch   | pacman   | pacman  |
| fedora | yum      | rpm     |
| fedora | dnf      | rpm     |
| gentoo | emerge   | portage |

> **Comandos de terminal** e **gerenciandores de pacotes** mais comuns no mundo linux

Um dos princípios da comunidade de **software livre** é a _liberdade de escolha_. Quando se trata de
_gerenciadores de pacote_, cada distro utiliza um específico.

## C, A Lingua Franca

Os programas básicos de todo sistema operacional moderno são todos escritos em **C**. Isso não é
apenas o reflexo de um legado de 50 anos, mas também algo intencial, devido a uma funcionalidade
única dessa linguagem de programação: A **vinculação dinâmica**. Essa funcionalidade permite
compilar um programa que depende de várias bibliotecas sem que elas estejam disponíveis em tempo de
compilação, apenas com os arquivos de **cabeçalho**.

Imagine por exemplo que no seu Linux, 50 programas usam uma biblioteca de criptografia chamada
**libre_crypto**. Ao rodar a atualização de sistema pelo gerenciador de pacotes, essa biblioteca é
atualizada da versão **2.11** para a versão **2.12**. Todos os 50 programas vão utilizar essa
biblioteca atualizada, com o bônus da correção dos bugs e o ônus de possíveis novos bugs.

Na prática, é um pouco mais complicado, já que cada gerenciador de pacotes possui uma estratégia
diferente de empacotamento para minimizar quebras de compatibilidade com atualizações. Para
aprofundamento, eu recomendo da leitura do excelente artigo
[symlinks and .so files on linux - what you need to know](https://dmerej.info/blog/post/symlinks-and-so-files-on-linux/).

### Analogia

É possível comparar esse funcionamento com o **NodeJS**. Imagine o seguinte import:

```js
import React from "react";
```

Quando o programa for executado, o **NodeJS** vai procurar pelo diretório **node_modules/react**.

De forma similar, quando um programa em **C** faz o seguinte include:

```c
#include <SDL2/SDL.h>
```

Quando o programa for executado, o **Sistema Operacional** vai procurar pelo arquivo
**/usr/lib/libSDL2.so**.

## O Pai De Todos

Vamos voltar brevemente para o _começo da História, 1º de janeiro de 1970_. Um novo Sistema
Operacional estava sendo criado na Bell Labs, o **UNIX**. Em pouco tempo, esse sistema passou a ser
usado em universidades por todos os Estados Unidos e logo várias pessoas passaram a criar o seu
próprio **UNIX**, com diversas distribuições sendo criadas. Como resultado, atualmente os principais
Sistemas Operacionais modernos **UNIX-like** de código aberto são:

- GNU/Linux
- OpenBSD
- FreeBSD
- Minix

Todos esses sistemas possuem **mais ou menos** as mesmas **APIs** disponíveis para qualquer
programa. O que significa que, qualquer programa escrito pensando no **GNU/Linux** pode ser portado
para o **OpenBSD** ou para o **FreeBSD**, ou vice-versa.

Isso implica que é possível rodar **ZSH**, **fastfetch**, e até mesmo **gnome** tanto em um
**GNU/Linux** quando no **OpenBSD**, e esses são _exemplos reais que eu encontrei na Internet_.

## Conclusão

A liberdade do software livre as vezes pode levar a problemas de compatibilidade e a bugs. Porém é
importante entender os desafios e as vantagens de manter a portabilidade dos programas entre
sistemas do UNIX-like.
