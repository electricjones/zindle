# Zindle
A configuration language for rust games. Temporary name. Experiment.

**Please don't use this for anything. It's a weekend experiment**

Zindle is a configuration system language meant to be used with Rust.
It's primary purpose is for a "House Rules" system for games using the Bevy engine.
In this system, the Game Developer can expose certain settings
and the Player can override those settings to mold the game rules as they wish.

## Concepts
There are 3 major pieces to the Zindle System:
1. [The Configuration](docs/configuration.md) is how the game developer exposes settings
and game rules. This is done in 100% valid Rust using macros.

2. [The Zindle Scripts](docs/zindle-lang.md) is the lightweight language used by game players.
This languages allows them to override settings, react to events, and record state.

3. [The Zindle Runtime](docs/runtime.md) is the Rust object that behaves as a virtual machine.
It parses, compiles, and executes scripts. It also exposes an API to the game to read properties.

### General Flow
The developer configures a `Configuration` object with all the settings they want to expose.
These settings are nested and may have validations, relationships, and defaults.
The developer also registers `Event`s that take place as well as `Context` objects that the scripts have access to.
Last, the developer configures the Runtime itself.

Next, a player (or AI, see below) creates `.zindle` scripts that override those settings.
These scripts can use primitive scalar values or can contain anonymous functions that
look at the state of the game to decide what a value is at any time. These properties can be
eagerly or lazily loaded, cached, and processed in parallel.

These scripts can also register events, which are functions that are called at certain points in gameplay.
These event functions can mutate only what the game developer has marked as mutable.

In all cases, there are defaults so that in case of error, a default is used.

Last, the `Runtime` is created when the game starts and parsed, compiles, and caches the scripts.
Then, the game can fetch settings from that Runtime, which will perform its evaluations and always return a value.

## Core Features
### The Configuration
- Setup using 100% valid rust with derive and procedural macros.
- Allow for nested properties and namespacing.
- Give each property a default value.
- Allow built in validation for things like numeric range.
- Allow custom validation on any property and chain validations for a single property.
- Strictly describe relationships between properties and validate those relationships.
- Extensive documentation on the Configuration
- Create `Event`s, which are moments scripts can respond to.
- Add `Context` so that any function/event can have access to the game state to make decisions.

### The Runtime
- Be immutable by default. The only values that can be mutated must be explicitly marked.
- Parse and validate scripts "just in time" and (through a cli) before hand.
- Be fast enough that it doesn't slow a game (caching, lazy, etc).
- Never panic -- always return a default value in case of error.
- Integrate with any compatible logging system.

### The Zindle Language
- Be simple and well defined. No extra features.
- Immutable by default. Only local variables and explicitly marked arguments can be mutated.
- Built in testing and documentation of scripts. And script validation.
- Allow namespacing and multiple script files.
- Annotations to add metadata and instructions to properties, events, and functions.
- No function ever has an unknown side-effect. They only return values.
- All functions are transactional. If any error, return the default value.
- Built in logging and error handling.
- Strongly typed with inferred types. Optional type annotation when possible.
- Specific language constructs like `event` and `override`.
- No concept of `null`. Use rust's option.
- Built in standard library with no side-effects.
- Primitive Types include collection types and strings, with built in manipulation.
- Never panics.

## Non Goals (at least for now)
- Zindle-lang as a full general purpose langauge or even a language as extensive as [Rhai](http://rhai.rs).
- Extreme performance (at first).
- Debugging, IDE support, Language server.

## Examples

## AI Generation
One of the primary goals of Zindle is that it is consice, well defined, and simple enough
that a well-trained Large Language Model can generate the configuration scripts.

This would allow (eventually) for something like this:

```
> Player
Add a house rule that all players start with 20 dollars instead of 10.

> Zindle
Done. Now all players will start with $20.

> Player
I don't want anyone to be eliminated. So, when a player reaches 1 dollar, give all players 3 dollars.

> Zindle
Done, now no player will "go broke" and be eliminated, but all players will get the money.

> Player
I want all the "Poison" spell cards to be "Flower" cards instead.

> Zindle
Unfortunately there is no mechanism in the game mechanics that let me change that rule.
But I can change the value of the Poison Cards.

> Player
Okay, can you set the value to 0? And then let the user draw a new card?

> Zindle
Yes, I have set all the Poison Cards to be Zero and whenver a Poison card is drawn the Player draws a free card.
```

### Foundation for Bigger Goals
This basic Zindle implementation sets the stage for 2 other features, maybe as different packages.
1. Full Game Creation (within reason) using text prompts. 
Allows a designer to describe game rules and upload or generate art. The AI then creates the skeleton.
This is interactive, like Star Trek's Holodeck.

2. A Game Helper that knows the rules and can answer rule questions and give suggestions based on the current state.




