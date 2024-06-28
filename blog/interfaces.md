# Interfaces

## Data types

Let's consider programming in terms of types. A number can be an integer, a float, or a type with specific precision. A text can be a character, a string, or a binary format with an unspecified size. Likewise, an object's behavior can be defined by a type, referred to as an _interface_ in most OOP languages.

An interface defines a set of operations with arguments and return types. This allows for:
  - Polymorphism
  - Multiple implementations
  - Plain stub objects for testing

## Applying interfaces

I implemented the PreciseSchedule's backend using as many interfaces as possible. In the process, I found out that: **the more interfaces you have, the harder it is to understand the code intentions**. In my case, the goal was to provide a default implementation and a default stub for testing.

When applying this approach to a layered clean architecture, it becomes hard to follow the code direction, the code operations and their results. The code becomes bloated with type definitions and stubs. Many tests  assert object interations instead of value assertions.

I also found that there is two problems that some languages type system do not cover:
  - Null values
  - Error handling

## Language by language

### Java

Modern Java provides:

- `@Nullable` and `@NotNull` annotations to handle null values
- `throws` keyword to make error handling explicit
- _Mockito_ and similar libraries to mock dependencies during runtime.

### Typescript

Typescript allows for union types, so a variable can be, for instance `string | number | null | undefined` (string or number or null or undefined). This type system allows handling empty values explicitly

### Rust

Is a language that provides a simple type system, that is enhanced by the standard library, by the following data structures:

- _Option_ to handle nullable values
- _Result_ to handle errors


## Alternatives

The Rust programming language provides  and  data

 another alternative I like the most with features like _Option_ and _Result_ from the Rust programming language solve the problem of explicitly managing errors and null values in your code.
