# Interfaces

## Data types

In any programming language, data has types. Numbers, text, binaries, all of them have a specific
type. The same way, an object can be defined by a type, referred to as an _interface_ in most
languages.

An interface is a set of operations with arguments and return types. This allows for:

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

Any functions that has a **CreateUserService** argument can be tested with createUserServiceStub and
createUserServiceErrorStub instead of mocking all dependencies. This helps focus on input and
output.

Any functions that has a **CreateUserService** argument can be tested using
**createUserServiceStub** and **createUserServiceErrorStub** instead of mocking all dependencies.
This abstracts implementation and let you think about **input** and **output**.

## Applying

If you apply interfaces exhaustively, the code indirection makes it hard to follow the code. So you
must minimize interfaces.

Another pitfall is that there is two problems that some languages type-system do not cover:
  - **Null values**
  - **Error handling**

## Language by language

### Java

Java provides:

- `@Nullable` and `@NotNull` annotations to handle null values
- `throws` keyword to make error handling explicit
- _Mockito_ and similar libraries to mock dependencies during runtime.

### Typescript

Typescript supports union types, so a variable can be, for instance
`string | number | null | undefined`. This makes empty values explicit.

### Rust

Rust offers a simple and powerful type system:

- No _null_ pointer
- _Option_ data structure to handle absent values
- _Result_ data structure to handle errors
