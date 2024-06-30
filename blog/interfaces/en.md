# Interfaces

Is there a way to make a _big project_ easy to _implement_, easy to _test_ and _type-safe_?

## Data types

In any programming language, data has types. Numbers, text, binaries, all of them have a _specific
type_. The same way, an object can be defined by a type, referred to as an _interface_ in most
languages.

> An interface is a set of operations with arguments and return types.

This allows for:

- Multiple implementations
- Hide dependency injection
- Plain objects for testing

## Why?

```ts
type CreateUserService = {
    create: (
        user: UserCreate,
        validator: UserValidator,
        idGenerator: IdGenerator,
        passwordGenerator: PasswordGenerator,
        repository: UserRepository,
    ) => Promise<User>;
};

export const createUserServiceActual: CreateUserService = {
    create: (
        user: UserCreate,
        validator: UserValidator,
        idGenerator: IdGenerator,
        passwordGenerator: PasswordGenerator,
        repository: UserRepository,
    ) => {/* omitted */},
};

export const createUserServiceStub: CreateUserService = {
    create: () => Promise.resolve(userStub),
};

export const createUserServiceErrorStub: CreateUserService = {
    create: () => {
        throw new Error();
    },
};
```

> Sample service to create a user, that also exports its _stubs_

Any functions that has a **CreateUserService** argument can be tested using
**createUserServiceStub** and **createUserServiceErrorStub** instead of mocking all dependencies.
This abstracts implementation and let you think about **input** and **output**.

## Pitfalls

- If you apply interfaces exhaustively, the code indirection makes it hard to follow the code. As a
  rule of thumb, use interfaces for: **external resources** (HTTP requests, database connections...)
  and **dependency injection** (When these resources are passed as arguments of functions, exactly
  as in the previous example).

- There is two problems that a type-system may not cover:

  - **Null values**
  - **Error handling**

## Language by language

Take the following example in _JavaScript_:

```js
async function createUser(
    user,
    repository,
) {
    if (!repository) {
        throw new Error("repository is required!");
    }
    return repository.save(user);
}
```

In order to compare this code to another languages, suppose:

- `repository` can be null
- `repository` can throw an error
- `user` is an required argument
- If `user` is correctly saved, the saved user is returned

### Typescript

Typescript supports union types, that allow typing required and null values:

```ts
function createUser(
    user: User,
    repository: UserRepository | null,
): DBUser {
    if (!repository) {
        throw new Error("repository is required!");
    }
    return repository.save(user);
}
```

### Java

Java provides:

- Annotation syntax that allows for `@Nullable` and `@NotNull` (There are many implementations from
  different libs)
- `throws` keyword to make error handling explicit
- _Mockito_ and similar libraries to mock injected dependencies during runtime

```java
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

public DBUser createUser(
    @NotNull User user,
    @Nullable UserRepository repository
) throws RepositoryException, Exception {
    if (repository == null) {
        throw new Exception("repository is required!");
    }
    return repository.save(user);
}
```

### Rust

Rust has a unique type-system because it has:

- No _null_ pointer
- _Option_ data structure to handle present or absent values
- _Result_ data structure to handle success or error values

The _Option_ and _Result_ work like enums, so it is necessary to handle each case:

```rs
pub fn create_user(
    user: User,
    repo: Option<UserRepo>,
) -> Result<DBUser, RepoErr> {
    match repository {
        Some(r) => Ok(r.save(user)),
        None => Err("Repository cannot be null"),
    }
}
```

## Conclusion

Interfaces, like any **abstraction**, can _hide errors_. If null and errors are
**explicit**, these errors can be avoided, at the cost of verbose code. Personally, I like the
**Rust** solution.
