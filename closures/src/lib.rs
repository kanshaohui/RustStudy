use std::thread;
use std::time::Duration;
use std::collections::HashMap;

pub struct Cacher<T>
    where T: Fn(u32) -> u32
{
    pub calculation: T,
    pub value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    pub fn value(&mut self, intensity: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(intensity);
                self.value = Some(v);
                v
            },
        }
    }

    pub fn value_hash(&mut self, intensity: u32, value_saved_hash: &mut HashMap<u32, u32>) {
        value_saved_hash.entry(intensity).or_insert((self.calculation)(intensity));
    }
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);


    let expensive_cloures = |num| {
        println!("Calculating slowly......");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut v = Cacher::new(expensive_cloures);
    let mut cacher_saved = HashMap::new();
    v.value_hash(intensity, &mut cacher_saved);
    let v1 = match cacher_saved.get(&intensity) {
        Some(va) => va,
        None => &0u32,
    };

    if intensity < 25 {
        println!("Today, do {} pushups", v1);
        println!("Next, do {} situps!", v1);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today run for {} minutes.", v1);
        }
    }
}

#[cfg(test)]
#[test]
fn call_with_diffrent_value() {
    let mut cacher = Cacher::new(|x| x);
    let mut hash_saved = HashMap::new();
    cacher.value_hash(1, &mut hash_saved);
    cacher.value_hash(2, &mut hash_saved);
    let v1 = match hash_saved.get(&1) {
        Some(va) => va,
        None => &0u32,
    };

    let v2= match hash_saved.get(&2) {
        Some(va) => va,
        None => &0u32,
    };

    for (key, item) in hash_saved.iter(){
        println!("{} => {}", key, item);
    }
    assert_eq!(2, *v2);
}