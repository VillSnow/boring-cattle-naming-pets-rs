use rand::{rng, seq::IndexedRandom};

pub fn get_random_name(format: &str) -> String {
    let mut rng = rng();
    let adjectives = include_str!("boring-cattle-naming-pets-words/adjectives.txt")
        .lines()
        .collect::<Vec<_>>();
    let nouns = include_str!("boring-cattle-naming-pets-words/nouns.txt")
        .lines()
        .collect::<Vec<_>>();
    let verbs = include_str!("boring-cattle-naming-pets-words/verbs.txt")
        .lines()
        .collect::<Vec<_>>();

    let a = adjectives.choose(&mut rng).unwrap();
    let [b, d] = nouns.sample_array(&mut rng).unwrap();
    let c = verbs.choose(&mut rng).unwrap();

    let result = format;
    let result = result.replace("boring", a);
    let result = result.replace("cattle", b);
    let result = result.replace("naming", c);
    let result = result.replace("pets", d);

    let result = result.replace("Boring", &title_case(a));
    let result = result.replace("Cattle", &title_case(b));
    let result = result.replace("Naming", &title_case(c));
    let result = result.replace("Pets", &title_case(d));

    let result = result.replace("BORING", &a.to_uppercase());
    let result = result.replace("CATTLE", &b.to_uppercase());
    let result = result.replace("NAMING", &c.to_uppercase());
    let result = result.replace("PETS", &d.to_uppercase());

    result
}

fn title_case(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            result.extend(c.to_uppercase());
        } else {
            result.push(c);
        }
    }
    result
}
