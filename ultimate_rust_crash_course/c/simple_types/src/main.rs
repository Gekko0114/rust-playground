// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]
use simple_types::{print_difference, print_array, ding, on_off, print_distance};


fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    for x in 0..series.len() {
        ding(series[x])
    }

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}