# Zindle
A configuration language for rust games. Temporary name. Experiment.

**Please don't use this for anything. It's a weekend experiment**

Zindle is a configuration system language meant to be used with Rust.
It's primary purpose is for a "House Rules" system for games using the Bevy engine.
In this system, the Game Developer can expose certain settings
and the Player can override those settings to mold the game rules as they wish.

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




