## ToDo
- 
- Change anonymous functions to lamdas and name all of them collectively "routines"

## Chat Notes for improvement
The specifications for the Zindle language seem well-detailed and thoughtfully structured. They lay a solid foundation for developers and AI to work within a defined framework while writing or generating game configurations. Here are some feedback and considerations regarding the specifications:

    Type Handling:
        The language's type system seems well defined. However, the mapping between the Rust types in the Configuration setup and the Zindle's primitive and composite types might need to be explicitly defined to ensure seamless integration.

    Function and Event Transactionality:
        The transactional nature of functions and events is a good strategy to maintain consistency and reliability. It might be beneficial to clearly define what constitutes a transaction failure and how transaction rollbacks are handled.

    Error Handling:
        The simplified error handling using Rust's Result and Option types is a practical approach. It could be beneficial to provide a mechanism for logging or tracking errors, especially during the development or debugging phase.

    Testing:
        The support for doctests and standalone tests is crucial for verifying the correctness of scripts. It may also be useful to consider some form of integration or end-to-end testing framework, especially to test interactions between different scripts and events.

    Documentation:
        Encouraging comprehensive documentation is vital for readability and maintainability. It might be worth considering a standardized format or template for documentation to ensure consistency across different scripts.

    Standard Library:
        The standard library's scope seems practical and useful. It's important to ensure that the library functions are well-documented and tested.

    Namespaces and Overrides:
        The mechanism for namespacing and overrides is well thought out. It might be useful to have a clear error reporting mechanism for unauthorized overrides or namespace violations, which could help in debugging and maintaining the script integrity.

    Runtime Environment:
        While the specifications cover the language syntax, types, and structures in detail, there might be a need to define how the Zindle scripts will be executed within the runtime environment, how they interact with the game state, and how the runtime environment handles script failures or errors.

    Versioning and Upgradability:
        It might be beneficial to consider a versioning mechanism for the Zindle language and scripts to ensure backward compatibility and smooth upgrades as the language evolves over time.

    Performance Considerations:
        Some performance considerations might be necessary, especially when dealing with complex games with numerous scripts and events. Defining some performance benchmarks or guidelines could be beneficial.

    Tooling:
        Consideration for tooling, such as a script editor, debugger, or a linter could also be beneficial to provide a more developer-friendly environment.

These considerations aim to provide a more comprehensive, robust, and developer-friendly framework for game configuration scripting using Zindle.

## Scraps
```
ad_code)]
use std::collections::HashMap;

pub trait Map<K, V, P> {
    // First
    fn get(&self, key: &K) -> Option<&V>;
    fn property(&self, key: &String) -> Option<&P>;
    fn contains_key(&self, key: &K) -> bool;
    fn insert(&mut self, key: K, property: P) -> Option<P>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn remove(&mut self, key: &K) -> Option<P>;

    // Second
    // fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    // fn iter(&self) -> Iter<K, V>;
    // fn iter_mut(&mut self) -> IterMut<K, V>;
    // fn try_insert(&mut self, key: K, value: V) -> Result<(), (K, V)>;

    // Third
    // fn entry(&mut self, key: K) -> Entry<K, V>;
    // fn extract_if<F>(&mut self, pred: F);
    // fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
    // fn into_keys(self) -> Keys<K, V>;
    // fn into_values(self) -> Values<K, V>;
    // fn keys(&self) -> Keys<K, V>;
    // fn remove_entry(&mut self, key: &K) -> Option<(K, V)>;
    // fn values(&self) -> Values<K, V>;
    // fn values_mut(&mut self) -> ValuesMut<K, V>;

    // Probably not
    // fn capacity(&self) -> usize;
    // fn clear(&mut self);
    // fn drain(&mut self);
    // fn get_many_mut<Q: ?Sized>(&mut self, keys: &[&Q]) -> Vec<Option<&mut V>>;
    // fn get_many_unchecked_mut(&mut self, keys: &[&K]) -> Vec<&mut V>;
    // fn hasher(&self) -> &S;
    // fn raw_entry(&self) -> RawEntry<K, V, S>;
    // fn raw_entry_mut(&mut self) -> RawEntryMut<K, V, S>;
    // fn reserve(&mut self, additional: usize);
    // fn retain<F>(&mut self, f: F);
    // fn shrink_to(&mut self, min_capacity: usize);
    // fn shrink_to_fit(&mut self);
    // fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
    // fn with_capacity(capacity: usize) -> Self;
    // fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self;
    // fn with_hasher(hasher: S) -> Self;
}

pub struct Annotation {}

pub enum Instruction {
    Add,
    Multiply,
    // etc
}

enum RoutineKind {
    Function(String),
    Lambda,
    Event(String),
}

pub struct Argument {
    name: String,
    kind: String,
}

pub struct Routine {
    bytecode: Vec<Instruction>,
    value: Option<Box<Value>>,
    arguments: Vec<Argument>,
    kind: RoutineKind,
    annotations: Vec<Annotation>,
}

pub enum Value {
    Int(i128),
    Uint(u128),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Dictionary(Box<dyn Map<String, Value, Property>>),
    Tuple(Vec<Value>),
    Routine(Routine),
}

pub struct Property {
    pub value: Value,
    annotations: Vec<Annotation>,
    name: String,
    path: String, // Fully qualified namespace
}

pub struct Collection {
    inner: HashMap<String, Property>,
}

impl Collection {
    fn new() -> Self {
        Collection {
            inner: HashMap::new(),
        }
    }
}

impl Map<String, Value, Property> for Collection {
    /// Keys are dot notation: a.b.c
    /// Any None's will return None for the entire path
    /// Any sub dictionaries will return another instance of Dictionary, which will just be the Map
    fn get(&self, key: &String) -> Option<&Value> {
        // TODO: If value is a Routine, execute the routine and cache the result
        Some(&self.inner.get(key).unwrap().value)
    }

    fn property(&self, _key: &String) -> Option<&Property> {
        unimplemented!();
    }

    fn contains_key(&self, key: &String) -> bool {
        self.inner.contains_key(key)
    }

    fn insert(&mut self, key: String, property: Property) -> Option<Property> {
        self.inner.insert(key, property)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn remove(&mut self, key: &String) -> Option<Property> {
        self.inner.remove(key)
    }
}
```
