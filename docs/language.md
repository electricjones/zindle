# Zindle Language
The Zindle Language is designed to allow for a clear, concise, and intuitive way to define game configurations. 
The Zindle language design aims to provide a solid, testable, and well-documented platform for defining game configurations, ensuring both ease of use and reliability in modifying game parameters.

The key aspects include:

## Properties:
Properties are the basic building blocks in Zindle which hold values or expressions that define game parameters. 
They can be simple scalar values or result from anonymous function evaluations.

## Functions (Anonymous and Named):
Functions can be defined either anonymously or with names. They are pure, having no side effects, 
and can ask for specific contexts to operate. Named functions require a declared return type.
Anonymous function can only be used on properties and return the type of that property.
Named functions can mutate specific parameters, but only when declared.

## Events:
Events are special constructs that can be triggered during gameplay. 
They can hold logic to modify game states based on certain conditions. 
Events can be prioritized using metadata like #[priority: 5].

## Namespacing and Overrides:
Zindle scripts adhere to a namespace structure mirroring the configuration setup. 
Scripts can override properties outside their namespace using an override block or #[override] annotation.

## Annotations:
Annotations like #[run_once] and #[override] provide additional metadata or directives for the Zindle scripts, 
enhancing clarity and control.

## Loops and Control Flow:
Basic control flow constructs like loops and conditionals are provided to enable more complex logic in configuring game parameters.

## Testing:
The language supports testing through docblocks and standalone test constructs, ensuring reliability and correctness of the scripts.

## Documentation:
Documentation is an integral part of the Zindle design, aimed at providing clear explanations for each script, 
function, and property.

## Core Language Features:
Features like type inference, error handling using Rust’s Result and Option types, and support for literals and enums are 
included to ensure a robust and expressive language design.

## Standard Library:
A standard library providing common functionalities like math operations, string manipulations, 
and others is accessible within the Zindle. Standard library is immutable w/o side effects.

## Technical Specification
This technical specification outlines the syntax, types, control structures, and other fundamental aspects of the Zindle, providing a structured framework for scripting within the game environment. It also introduces advanced features like namespacing and annotations to facilitate more organized and controlled scripting.

### 1. Structure:
#### 1.1 Basic Elements:
- Properties: `property_name = value;`
    - Properties set configuration properties.
    - Properties are namespaced.
    - Properties have annotations
        - `#[override]` to modify a property outside this file's namespace
        - `#[run_once]` if the property is an anymous function that wants to run only once
        - `#[eager]` to run the attached function at startup instead of lazily (by default)
- Functions:
    - Named: `fn function_name(parameter: ParameterType, ...): return_type { ... }`
    - Anonymous: `property_name: (parameter: ParameterType, ...) { ... }` returns the type of the parameter
    - Functions are **transactional**, if any error occurs, no data is updated.
    - Functions are error safe. If any error occurs, a default value is returned.
        - The default value can be (optionally) passed into the function with a special `default: Default` parameter.
        - The default value can be set as an anotation `#[default = 5]`
        - A default value MUST be set on the configuration setup.
        - Defaults are resolved in that order, falling down until a default value is discovered.
- Events: `event EventName (parameter: ParameterType, ...) { ... }`
    - Events can have annotations
        - `#[priority: x]` sets the priority of the event
    - Events can mark certain parameters as mutable so they can update the game state `event Name (a: mut Type)`
    - That is only possible if the configuration marked that type as potentially mutable
    - Events MUST return any mutable types, either singlely or as a tuple (for multiple)
    - Events CANNOT modify any properties unless they request the `config: mut Config` object and update on that.
    - All events are transactional. Any failure causes all changes to be lost.

### 2. Syntax
#### 2.1 Basic Syntax:
- Variables
    - variables can be created in functions, events, and anonymous functions only.
    - `var name: Type` signifies a variable that is mutable.
    - `const name: Type` signifies a variable that is immutable.
- Conditional Expressions: `if expression { ... } else { ... }`. They are expressions and can be used as return values.
- Ternary Conditionals: `some_condition ? 10 : 30`;
- Null and False operator: `value ?? fall_back_value` with return the fallback value if the first value is `None` or evaluates to `false` or is an `Error`
- Loops:
    - Basic loop: `loop { ... }`
    - For loop: `for (element in collection) { ... }`
- Match Statement: `match expression { ... }`
    - Follows the conventions of a rust match statement (exhaustive)
- Comments: `# Single-line comment`

#### 2.2 Advanced Syntax:
- Override Mechanism:
    - Annotation: `#[override] config.namespace.property = value;` to set a property outside this file.
    - Block: `override { config.namespace.property = value; ... }`
- Error Handling: Result and Option types with ? operator for error propagation.
    - Every function/event/anonymous function can return either a concrete value or an `Error()` object.
    - Use the ? to propegate errors.
    - No custom errors. All errors are `Error()` types. (For now)
- Type Casting: `expression as type` with an `Error` for overflows.

### 3. Types:
#### 3.1 Primitive Types:
- int, uint (unsigned int), float, bool, string

#### 3.2 Composite Types:
- Arrays: `[]` are like rust Vectors
- Tuples: `()` are identical to rust tuples
- Dictionaries: `{}` operate like rust HashMaps

#### 3.3 Custom Types:
Custom types are defined in the Configuration Structure by the Game Designer.

#### 4. Namespaces:
- Explicit Namespace Declaration: Scripts may explicitly declare the namespace they operate within, aligning with the structure defined in the Configuration.
- Override Mechanism: Explicit mechanism to access or mutate properties outside the script's namespace.

#### 5. Testing:
- Doctests: Incorporated into the documentation block preceding function/event declarations.
- Standalone Tests: `test "Test Name" { ... }`

#### 6. Documentation:
Doc Blocks: Documentation blocks precede function, event, and property declarations, providing essential information and potentially doctests.

#### 7. Standard Library:
Library Object: library object provides access to a standard library of functions and utilities.

- library.math provides basic math utilities
- library.files provides read only file utilities
- library.network provides basic networking utilities (the only one with potential side-effects)

