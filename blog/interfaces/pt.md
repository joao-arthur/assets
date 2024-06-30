# Interfaces

Existe alguma forma de tornar um _projeto grande_ fácil de _implementar_, fácil de testar _testar_ e
com _segurança de tipo_?

## Tipos de dado

Em qualquer linguagem de programação, dados possuem tipo. Números, texto, binários, todos eles
possuem um _tipo específico_. Da mesma forma, um objeto pode ser definido por um tipo, referido como
_interface_ na maioria das linguagens.

> Uma interface é um conjunto de operações com argumentos e tipagem de retorno.

Isso permite:

- Múltiplas implementações
- Esconder a injeção de dependências
- Objetos planos para o teste

## Por que?

```ts
type CriarUsuarioServico = {
    criar: (
        usuario: UsuarioCriar,
        validador: UsuarioValidador,
        geradorId: GeradorId,
        geradorSenha: GeradorSenha,
        repositorio: UsuarioRepositorio,
    ) => Promise<User>;
};

export const criarUsuarioServicoActual: CriarUsuarioServico = {
    criar: (
        usuario: UsuarioCriar,
        validador: UsuarioValidador,
        geradorId: GeradorId,
        geradorSenha: GeradorSenha,
        repositorio: UsuarioRepositorio,
    ) => {/* omitted */},
};

export const criarUsuarioServicoStub: CriarUsuarioServico = {
    criar: () => Promise.resolve(userStub),
};

export const criarUsuarioServicoErrorStub: CriarUsuarioServico = {
    criar: () => {
        throw new Error();
    },
};
```

> Exemplo de um serviço para criar um usuário, que também exporta os seus _stubs_

Qualquer função que receba um argumento do tipo **CriarUsuarioServico** pode ser testada usando
**criarUsuarioServicoStub** e **criarUsuarioServicoErrorStub** ao invés de mockar todas as
dependências. Isso abstrai a implementação e permite você pensa sobre **entrada** e **saída**.

## Armadilhas

- Se você aplicar interfaces exaustivamente, a indireção do código torna difícil seguir o código.
  Como uma boa prática, use interfaces para: **recursos externos** (requisições HTTP, conexão com
  bancos de dados...) e **injeção de dependência** (Quando os recursos são passados via argumentos
  da função, exatamente como no exemplo anterior).

- Existe depois problemas que um sistema de tipos pode não cobrir:

  - **Valores null**
  - **Gerenciamento de erros**

## Linguagem por linguagem

Veja o seguinte exemplo em _JavaScript_:

```js
async function criarUsuario(
    usuario,
    repositorio,
) {
    if (!repositorio) {
        throw new Error("repositorio é obrigatório!");
    }
    return repositorio.salvar(usuario);
}
```

Para poder comparar esse código com outras linguagens, suponha:

- `repository` pode ser null
- `repository` pode lançar um erro
- `user` é um argumento obrigatório
- Se `user` for corretamente salvo, o usuário salvo é retornado

### Typescript

Typescript supporta tipos de união, o que permite tipar valores requiridos e nulos:

```ts
function criarUsuario(
    user: Usuario,
    repository: UsuarioRepositorio | null,
): BDUsuario {
    if (!repositorio) {
        throw new Error("repositorio é obrigatório!");
    }
    return repositorio.salvar(usuario);
}
```

### Java

Java provê:

- Sintaxe de anotação que permite `@Nullable` e `@NotNull` (Existem muitas implementações de
  diversas bibliotecas)
- Palavra reservada `throws` para tornar o gerenciamento de error explícito
- _Mockito_ e bibliotecas similares para mockar injeção de dependências em tempo de execução

```java
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

public BDUsuario criarUsuario(
    @NotNull Usuario usuario,
    @Nullable UsuarioRepositorio repositorio
) throws RepositorioException, Exception {
    if (repositorio == null) {
        throw new Exception("repositorio é obrigatório!");
    }
    return repositorio.salvar(usuario);
}
```

### Rust

Rust possui um sistema de tipos único, porque possui:

- Não há ponteiro _null_
- Estrutura de dado _Option_ para lidar com valores presentes ou ausentes
- Estrutura de dado _Result_ para lidar com valores de sucesso ou erro

_Option_ e _Result_ funcionam como enums, então é necessário lidar com cada caso:

```rs
pub fn criar_usuario(
    usuario: Usuario,
    repo: Option<UsuarioRepo>,
) -> Result<BDUsuario, RepoErr> {
    match repo {
        Some(r) => Ok(r.salvar(usuario)),
        None => Err("repositorio é obrigatório!"),
    }
}
```

## Conclusão

Interfaces, como qualquer **abstração**, pode _esconder erros_. Se null e erros são **explícitos**,
esses erros podem ser evitados, ao custo de código verboso. Pessoalmente, eu gosto da solução do
**Rust**.
