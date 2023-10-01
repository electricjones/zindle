#![allow(dead_code)]
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
