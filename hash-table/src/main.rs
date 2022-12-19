use fnv::FnvHasher;
use std::fmt::Debug;
use std::hash::Hasher;
use std::vec::Vec;

const MAX_LOAD: f64 = 0.75;

#[derive(Debug)]
struct Tabl<V: Clone + Default + Debug> {
    count: u64,
    entries: Vec<Entry<V>>,
}

#[derive(Default, Clone, Debug)]
struct Entry<V: Clone + Default + Debug> {
    key: String,
    value: V,
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(bytes);
    hasher.finish()
}

impl<'a, V: Clone + Default + Debug> Tabl<V> {
    fn new(capacity: u64) -> Self {
        Tabl {
            count: 0,
            entries: vec![Entry::<_>::default(); capacity as usize], // only gonna work on 64 bit machines hehe
        } // kind weird and inefficient that we have to use default entries. In C do we actaully take up the meory when we grow an array?
          // maybe it's at riskj of overwriting in C but not here?
    }

    fn increment_count(&mut self) {
        self.count += 1;
    }

    fn grow_capacity(&mut self) {
        let mut new_cap = 0;
        let cap = self.entries.capacity();
        if cap < 8 {
            new_cap = 8
        } else {
            new_cap = cap * 2;
        };

        // resize vector
        let difference = new_cap - self.entries.len();
        let additional = vec![Entry::<V>::default(); difference];
        self.entries.extend(additional);
    }

    fn find_index(&self, key: &str) -> u64 {
        let cap = self.entries.capacity() as u64;
        let mut index = fnv1a(key.as_bytes()) % cap;

        for _ in 0..cap {
            let entry = &self.entries[index as usize]; // only gonna work on 64 bit machines hehe
            if entry.key == key || entry.key.is_empty() {
                return index;
            }

            index = (index + 1) % cap;
        }

        panic!("Could not find index");
    }

    fn get_entry_mut(&mut self, index: u64) -> &mut Entry<V> {
        &mut self.entries[index as usize]
    }

    fn set(&mut self, key: &str, value: V) -> bool {
        let cap = self.entries.capacity() as f64;
        if (self.count + 1) as f64 > cap * MAX_LOAD {
            self.grow_capacity()
        };

        let index = self.find_index(&key);
        let entry = self.get_entry_mut(index);
        let is_new_key = entry.key.is_empty();

        entry.key = key.to_string();
        entry.value = value;

        if is_new_key {
            self.increment_count();
        };
        return is_new_key;
    }
}

fn main() {
    let x = fnv1a(b"goo");
    println!("{}", x);

    let table = &mut Tabl::new(4); // dont like that we have to call this mut
    for i in 0..25 {
        let s = i.to_string();
        let val = i + 1;
        table.set(&s, val);
    }

    println!("{:?}", table.entries);
}
