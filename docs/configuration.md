# Configuration
The Configuration Setup is envisioned to be structured using Rust, which allows for precise, 
strict typing and robust validation through annotations and macros.
It aims to provide a robust, well-structured, and well-documented foundation upon which the Zindle Language can operate 
to modify game parameters in a clear and predictable manner.

## Concepts

### Nested Configuration:
Configuration objects are organized in a nested structure reflecting different game components 
such as PlayerConfig, EnemyConfig etc., which are grouped under a main GameConfig structure.

### Validation:
Each configurable property can have a validator. 
Validators can be built-in (e.g., range(1..=7)) or custom functions (e.g., using = crate::validators::some_validator).
To implement multiple validators, create a custom one that can call `Validator::range()` or other built ins.

### Relationships:
Relationships between different properties are defined using annotations like #[relation]. 
These annotations help in understanding how properties relate to one another and can be used for validation.
These annotations are strictly for the User/AI and are not used by the runtime to enforce anything.

### Contexts:
A `Context` is an object that can be available to Zindle Script functions, events, and anonymous functions.
There are several built in that are always available.
The setup also allows for the addition of custom context objects that can be accessed by the Zindle scripts. 
This provides a mechanism to inject game-specific data or objects that can be utilized within the script.

### Events:
Events are defined in the configuration setup and are associated with specific contexts. 
Events can be triggered during gameplay, allowing for dynamic adjustments based 
on changing game state or player inputs.

### Namespacing:
Namespaces are used to organize properties and functionalities in a structured manner, 
mirroring the organization in the Zindle scripts. 
This allows for a clear separation and organization of different game components and functionalities.

### Documentation:
Comprehensive documentation is heavily emphasized to ensure clarity and ease of understanding for 
both human readers and AI. Every property, relationship, event, and validator is expected to be well-documented.

## Technical Specification
### 1. Configuration Structure Definition:
The primary configuration structure is defined in Rust with a series of 
nested structs encapsulating various game parameter configurations.

```rust
#[derive(Configuration)]
struct GameConfig {
    player: PlayerConfig,
    enemy: EnemyConfig,
    // ... other configurations ...
}
```

#### 1.1 Annotations:
- `#[derive(Configuration)]`: Marks the struct as the main configuration structure.
- `#[configurable]`: Marks a property as configurable.
- `#[namespace]`: Optionally rename a sub-namespace w/o changing the rust struct property. 
- `#[validate([...])]`: Specifies validation rules for a property.
- `#[default(value)]`: Specifies a default value for a property.
A default value is required, either by an annotation or implementing `Default`

#### 1.1 Nested Configuration Structures:
Nested structs allow for organized grouping of related configurations.

```rust
#[derive(Configuration)]
struct PlayerConfig {
    // ... property configurations ...
}

#[derive(Configuration)]
struct EnemyConfig {
    // ... property configurations ...
}

#[derive(Configuration)]
struct GameConfig {
    player: PlayerConfig,
    enemy: EnemyConfig,
}
```

### 2. Contexts:
Contexts are objects that are available to any Zindle Script function, event, or anonymous function.
Some contexts are provide as Built-In:
- Log: offers logging like `log.info("")`
- Config: is the main Configuration itself
- Self: Is the the script itself
- Library: Is the standard library object

#### 2.1 Custom Contexts
The Runtime can also have arbitray context objects attached and available.
```rust
let mut runtime = Runtime::new();
let state = GameState::new();

runtime.add_context("State", &state, false);
```

Will add the state variable to the context so that any script or function can type-hint it

```
function (self: Self, state: State) -> bool {}
```

Will be magically given the object representing the script itself and the state.

### 3. Events
Events and custom context objects are defined and added to the runtime, 
enabling contextual access and event-driven interactions within scripts.

```rust
let mut runtime = Runtime::new();
runtime.add_context("SomeType", some_concrete_object, true);
runtime.add_context("Player", some_other_concrete_object, true);
runtime.add_event("EventName", ["SomeType", "Player"]);
```

This allows a script to create an:

```
event EventName (self: Self, log: Log, a: SomeType, player: Player) {}
```

Any events of this type have access to the normal built-in contexts and any custom contexts.
But only "SomeType" and "Player" will be mutable and those are expected to be returned. (see 3.1)

#### 3.1 Mutable Context and Events
For events to be useful, they must be able to make some changes.
For this to work, the event can ask for mutable context objects, make its updates, and then return those objects.
But, those requested mutable object MUST:
1. Be mutable when registered in rust (reccomended to use Rc or similar)
2. Must be registered by add_context() as allowed to be mutable `add_context("Name", object, true)`
3. Must be requested by the script as mutable `event EventName(player: mut Player)`
4. Must be returned by the event. `return player`

### 4. Validation:
Each property is automatically validated against its type.

#### 4.1 Built In Validators
Validation rules are specified through annotations on properties, 
using a combination of built-in and custom validation functions.

```rust
#[validate(range(1..=100))] // must be an integer between 1 and 100 (inclusive)
```

Built in validators include:
- range
- is_positive
- is_negative

#### 4.2 Custom Validators:
Custom validation functions can be defined and specified in the #[validate] annotation.

```rust
fn validate_players(number: u8) -> bool {
    return number < 10;
}

#[derive(Configuration)]
struct GameConfig {
    #[configurable]
    #[validate(using = validate_players)]
    player_count: u8,
}
```

### 5. Error Handling
All Zindle scripts are error free, since they mostly just set properties.
On encountering any error, the default value is set (and potentially logging occurs).
Error propogation with `?` is encouraged. And, many of rusts helpers are included (unwrap, etc).

### 5. Documentation:
Documentation is an integral part of the configuration setup, providing clear descriptions and explanations for each configuration property, event, and context object.

```rust
/// Some documentation about this property
#[configurable]
...
