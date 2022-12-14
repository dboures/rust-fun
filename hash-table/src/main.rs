
use std::vec::Vec;
use std::hash::Hasher;
use fnv::FnvHasher;

const MAX_LOAD:f64 = 0.75;

struct Tabl<'a, V>  {
    count: u64,
    entries: Vec<Entry<'a, V>>
}

struct Entry<'a,V> {
    key: &'a str,
    value: V
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(bytes);
    hasher.finish()
}



impl<'a, V> Tabl<'a, V> {
    fn new(capacity: u64) -> Tabl<'a, V> {
        Tabl {
            count: 0,
            entries: Vec::with_capacity(capacity.try_into().unwrap())
        }
    }

    fn find_entry(&self, key: &str) -> &Entry<'a, V> {
        let cap = self.entries.len() as u64;
        let mut index = fnv1a(key.as_bytes()) % cap; 

        loop {
            let entry = &self.entries[index as usize];// only gonna work on 64 bit machines hehe
            if entry.key == key || entry.key.is_empty() {
                return entry
            }

            index = (index + 1) % cap;
        }

    }

    // fn set<V>(&mut self, key: &str, value: V) -> bool {


        
    // }

}

fn main() {
    let x = fnv1a(b"goo");
    println!("{}", x);
}
