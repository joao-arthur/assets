# Libre Game Of Life

> O Jogo Da Vida é um autômato celular criado pelo matemático britânico John Horton Conway em 1970

Esse autômato acontece em uma grade onde cada celula pode estar ou viva ou morta. A cada passo de
tempo, o grupo de células vivas é determinada pelas seguintes regras:

- Celulas vivas sobrevivem com 2 ou 3 vizinhos vivos
- Celulas mortas se tornam vivas com 3 vizinhos vivos

## Implementação

### A Abordagem Por Array

A ideia dessa abordagem é guardar o estado de **todas** as células.

```ts
const grade = [
    [0, 1, 0],
    [1, 1, 1],
    [0, 1, 0],
];
```

No caso de uma grade quadrada, o **número** de itens do array vai ser igual ao **quadrado** do
**tamanho** da grade:

- Grade 10x10 = 100 itens
- Grade 100x100 = 10.000 itens
- Grade 300x300 = 90.000 itens

> Isso implica que o uso da memória cresce mais rápido que o tamanho da grade

#### O Problema Visual

Se todas as células estão confinadas **apenas dentro** das fronteiras da grade, elas ficam
**presas** nas bordas. Por exemplo, um glider em uma borda se **torna um block**.

![glider se transforma em um block](/images/glider_to_block.gif)

#### O Problema de performance

Criar uma nova geração obriga criar um novo array de mesmo tamanho do atual.

Você pode escolher entre:

- Iterar sobre todos os itens do array atual
  - **Vantagem**: Simplicidade
  - **Desvantagem**: Performance mais lenta com grades maiores
- Mapear as posições das células vivas e iterar sobre seus vizinhos
  - **Vantagem**: Performance melhorada com menos células vivas
  - **Desvantagem**: Mais complexo de implementar

### A abordagem do Map

A ideia dessa abordagem é guardas apenas o estado das células **vivas**.

```ts
const grade = new Map([
    ["(0, 1)", 1],
    ["(-4, 2)", 1],
    ["(7, -3)", 1],
]);
```

> Um _Set_ iria ser suficiente **nesse cenário**, já que uma célula possui apenas um estado

Uma mudança notória é que, com um _Map_, nós não podemos mais contar os índices do array. Ao invés
disso, precisamos usar um método diferente para identificar as posições das células, i.e., um
**sistema de coordenadas**. Eu escolhi o **plano cartesiano**.

#### A Solução Visual

Dado que um sistema de coordenadas é **infinito**, a UI funciona como uma **lupa**: Revelando apenas
uma pequena porção da paisagem, com a habilidade de se mover ao redor e aumentar ou diminuir o zoom.
A limitação nas bordas não existe mais:

![glider indo embora](/images/glider_away.gif)

#### A solução de performance

Para criar uma nova geração, um novo _Map_ deve ser criado, iterando sobre as células vivas e seus
vizinhos. Essa operação é independente do tamanho da UI e portanto, **muito mais rápida**.

## Aplicação Final

[Click aqui para ver!](/game-of-life/index.html)
