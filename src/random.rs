use rand::seq::SliceRandom;

pub fn random_different_index<T>(items: &mut Vec<T>, current_idx: usize) -> usize{
    let mut indexes: Vec<_> = (0..items.len()).collect();
    if let Some(pos) = indexes.iter().position(|v| *v == current_idx) {
        indexes.remove(pos);
        if let Some(idx) = indexes.choose(&mut rand::thread_rng()) {
            *idx
        } else {
            eprintln!("was given {} items, choose_random() is useless.", items.len());
            current_idx
        }
    } else {
        eprintln!("was given {} items, but current_idx is {}", items.len(), current_idx);
        current_idx
    }
}
