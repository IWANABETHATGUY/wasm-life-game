use wasm_game_of_life::Universe;
fn transform_string(arr: &mut Vec<char>) {
    let mut idx = arr.len();
    for i in (0..arr.len() - 1).rev() {
        match arr[i] {
            'A'..='Z' =>  {
                arr.insert(idx, arr[i]);
                arr.remove(i);
                idx -= 1;
            }
            _ => {}
        }
    }
}
fn main() {
    let mut vec = String::from("OkhaoPingCeilXu")
        .chars()
        .collect::<Vec<char>>();
    transform_string(&mut vec);
    println!("{:?}", vec);
}
