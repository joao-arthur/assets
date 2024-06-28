# Interfaces

## Data types

In any programming language, we handle data that has a type. Numbers, text, binaries, they have types. The same way, an object's behavior can be defined by a type, referred to as an _interface_ in most languages.

An interface defines a set of operations with arguments and return types. This allows for:
  - Multiple implementations
  - Hide dependency injection
  - Plain stub objects for testing

## Example

A user create service would look like this:

```typescript
type CreateUserService = {
    create: (
        user: UserCreate,
        modelValidator: UserModelValidator,
        uniqueUsernameService: UniqueUsernameService,
        idGenerator: IdGenerator,
        passwordGenerator: PasswordGenerator,
        repository: UserRepository,
    ) => Promise<User>;
}

export const createUserServiceActual: CreateUserService = {
    create: (
        user: UserCreate,
        modelValidator: UserModelValidator,
        uniqueUsernameService: UniqueUsernameService,
        idGenerator: IdGenerator,
        passwordGenerator: PasswordGenerator,
        repository: UserRepository,
    ) => { /* omitted */ }
};

export const createUserServiceStub: CreateUserService = {
    create: () => Promise.resolve(userStub)
};

export const createUserServiceErrorStub: CreateUserService = {
    create: () => { throw new Error(); }
};
```

Now, any function that receives a **CreateUserService** can be tested using **createUserServiceStub** and **createUserServiceErrorStub** instead of mocking all those dependencies. This abstracts implementation to let you think about **input** and **output**.

## Applying

If you apply interfaces exhaustively, the code indirection makes it hard to follow the code. So you must minimize interfaces.

Another pitfall is that there is two problems that some languages type-system do not cover:
    - **Null values**
    - **Error handling**

## Language by language

### Java

Modern Java provides:

- `@Nullable` and `@NotNull` annotations to handle null values
- `throws` keyword to make error handling explicit
- _Mockito_ and similar libraries to mock dependencies during runtime.

### Typescript

Typescript allows for union types, so a variable can be, for instance `string | number | null | undefined` (string or number or null or undefined). This type system allows handling empty values explicitly.

### Rust

Is a language that provides a simple and powerful type system:

- No _null_ pointer
- _Option_ data structure to handle absent values
- _Result_ data structure to handle errors
