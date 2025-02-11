# Thoughts on FOSS and UNIX and community

Unix não é um sistema operacional. unix pode ser definido como o conjunto da especificação unix com a especificação posix. os sistemas unix-like dominam o Mercado em todas os ambientes, exceto ambientes desktop.

Quando você entra na comunidade linux, você vai instalar softwares usando apt, pacman, rpm ou portage. para cada uma dessas distribuições existem detalhes de como empactoar o software, mas de forma geral todos eles vão depender de uma funcionalidade chamada dynamic linking. isso se deve por conta de que os softwares são escritos em C. isso permite que você atualize uma biblioteca sem precisar recompilar o sistema inteiro. Isso permite corresções de bug, novas funcionalidades e novos bugs. para o usuário médio isso é transparente, já que esses problemas são mais comuns em bleeding edge.

ou seja, como tudo na vida, existe um balanço de vantagens e desvantagens.



unix can be defined as the joint of the posix and unix specifications. unix-like systems dominate all ecossystems, but desktop environment.

when you get into unix you need to compile the software using c. the thing with C is that it is a flexibe language, that allows dynamic linking, and that is very useful for operational systems, specially, because it allows for shorter and more efficient binaries, and avoid recompile the entire system when there is a small change.

but, at the same time, unexpected errors can occur when a library changes, specially if you are running on bleeding edge.

more than that, it became such a standard that most linux software is made for specific distributions, using the 