# Linux, UNIX, pacotes e bugs

Por que as vezes, quando se usa Linux, tudo parece bagunçado e nada funciona? Por que toda a
comunidade de software livre ainda usa C?

## Abram alas para o Linux

Generalizando, toda pessoa que começa no **Linux** instala uma distribuição famosa. A partir daí ela
vai instalar seus aplicativos a partir de comandos. Normalmente, esses comandos são apenas **CLIs**,
que agem de **frontend** para programas mais complexos.

| distro | frontend | backend |
| ------ | -------- | ------- |
| debian | apt      | dpkg    |
| arch   | pacman   | pacman  |
| fedora | yum      | rpm     |
| fedora | dnf      | rpm     |
| gentoo | emerge   | portage |

> Tabela dos **comandos de terminal** e **gerenciandores de pacotes** mais comuns no mundo linux

Um dos principios da comunidade de **software livre** é a _liberdade de escolha_. Quando se trata de
_gerenciadores de pacote_ a questão não é tão simples.

## C, a lingua franca

Os programas básicos de todo sistema operacional moderno é escrito em **C**. Isso não é tão-somente
o reflexo de um legado de 50 anos, mas sim algo intencional, devido a uma funcionalidade única dessa
linguagem de programação: A **Vinculação dinâmica**. Essa funcionalidade permite compilar um
programa que depende de uma certa biblioteca sem que ela esteja disponível em tempo de compilação. O
que permite isso é a existência de arquivos de **cabeçalho**.

Imagine por exemplo que no seu Linux, 50 programas usam uma biblioteca de criptografia chamada
**libre_crypto**. Ao rodar a atualização de sistema pelo gerenciador de pacotes, essa biblioteca é
atualizada da versão **2.11** para a versão **2.12**. Todos os 50 programas vão utilizar essa
biblioteca atualizada, com o bônus da correção dos bugs e o ônus de novos possíveis bugs.

Na prática, é um pouco mais complicado, já que, cada gerenciador de pacotes possui uma estratégia
diferente de empacotamento para minimizar bugs e quebras de compatibilidade. Para aprofundamento, eu
recomendo da leitura do excelente artigo
[symlinks and .so files on linux - what you need to know](https://dmerej.info/blog/post/symlinks-and-so-files-on-linux/).

## O pai de todos

Vamos voltar brevemente para o _começo da História, 1º de janeiro de 1970_. Um novo Sistema
Operacional estava sendo criado na Bell Labs, o **UNIX**. Esse sistema passou a ser usado em
universidades por todos os Estados Unidos e logo várias pessoas passaram a querer criar o seu
próprio **UNIX**. Como resultado, atualmente os principais sistemas operacionais modernos do
**tipo-UNIX** de código aberto são:

- GNU/Linux
- OpenBSD
- FreeBSD
- Minix

Todos esses sistemas possuem **mais ou menos** as mesmas funcionalidades disponíveis para qualquer
programa. O que significa que, qualquer programa escrito pensando no **GNU/Linux** pode ser portado
para o **OpenBSD** ou para o **FreeBSD**, ou vice-versa.

Leia novamente a última frase. Ela implica que você pode rodar **ZSH**, **fastfetch**, e até mesmo
**gnome** tanto em um **GNU/Linux** quando no **OpenBSD**, e esses _não são exemplos hipotéticos_.

## Conclusão

A liberdade do software livre implica a liberdade de escolha, o que as vezes pode levar a problemas de compatibilidade e a bugs. Porém é um erro acreditar que o Linux está errado, sem entender mais profundamente os desafios e as vantagens de como as coisas funcionam hoje em dia.