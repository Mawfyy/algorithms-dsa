use std::fmt::Display;

#[derive(Debug)]
pub struct Hashmap<V> {
    values: Vec<Values<V>>,
    capacity: usize,
}

#[derive(Debug)]
pub enum Values<V> {
    GarbageValue,
    Value(V),
}

impl<V: Display> Hashmap<V> {
    pub fn new(capacity: usize) -> Self {
        let mut vector = Vec::<Values<V>>::with_capacity(capacity);

        for _ in 0..capacity {
            vector.push(Values::GarbageValue);
        }

        Hashmap {
            values: vector,
            capacity,
        }
    }

    pub fn insert(&mut self, key: String, value: V) {
        let mut index = calculate_index(key.clone(), self.capacity);

        if let Values::Value(_) = self.values[index] {
            while index <= self.values.len() - 1 {
                index += 1;
                if let Values::GarbageValue = self.values[index] {
                    break;
                }
            }
        }

        if index == self.values.len() - 1 {
            let mut current_index = 0;
            let target_index = calculate_index(key, self.capacity);
            while current_index <= target_index {
                current_index += 1;
                if let Values::GarbageValue = self.values[current_index] {
                    break;
                }
            }

            self.values.insert(current_index, Values::Value(value));
            return;
        }

        self.values.insert(index, Values::Value(value))
    }

    pub fn get(&self, key: String) -> &Values<V> {
        let index = calculate_index(key, self.capacity);
        &self.values[index]
    }

    pub fn exists(&self, key: String) -> bool {
        let index = calculate_index(key, self.capacity);
        if let Values::GarbageValue = &self.values[index] {
            return false;
        }

        true
    }

    pub fn remove(&mut self, key: String) {
        let index = calculate_index(key, self.capacity);
        self.values.remove(index);
    }
}

pub fn calculate_index(key: String, capacity: usize) -> usize {
    key.len() % (capacity - 1)
}
