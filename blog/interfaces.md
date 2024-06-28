# Interfaces

## Data types

Let's consider programming in terms of types. For instance, a number can be an integer, a float, or a type with specific precision. A text can be a character, a string, or a binary format with an unspecified size. Likewise, an object's behavior can be defined by a type, referred to as an _interface_ in most OOP languages.

An interface defines a set of operations with arguments and return types. This allows for:
  - Polymorphism
  - Highly decoupled implementations
  - Plain stub objects

## Applying interfaces

I implemented the PreciseSchedule's backend using as many interfaces as possible. In the process, I found out that: **the more interfaces you have, the harder it is to understand the code intentions**. In my case, the goal was to provide a default implementation and a default stub for testing.

When applying this approach to a layered architecture, there is too much indirection. It is hard to follow the code direction, therefore, the code operations and their results. The code becomes bloated with type definitions and stubs. The tests only assert object interations instead of value assertions. If your programming language doesn't make error handling explicit, you will forget to handle an exception sooner or later.

## Alternatives

What I found is likely something Java developers realized a couple of years ago with libraries like _Mockito_: rather than using an interface solely for testing purposes, you can mock your code's dependencies during runtime testing.

At this point, I'm inclined to believe that features like _Option_ and _Result_ from the Rust programming language solve the problem of explicitly managing errors and null values in your code.

In the future, I intend to adopt a _default approach_, creating interfaces only for resources such as databases, random data generation, and HTTP requests.
