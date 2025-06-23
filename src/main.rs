#![allow(unused)]

fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 2) + fib(n - 1) }
}

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let [[a, b, c], [d, e, f], [g, h, i]] = matrix;
    [[a, d, g], [b, e, h], [c, f, i]]
}

#[allow(clippy::needless_range_loop)]
fn transpose2(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(v: &[f64; 3]) -> f64 {
    let mut r = 0.0;
    for elem in v {
        r += elem * elem;
    }
    r.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(v: &mut [f64; 3]) {
    let m = magnitude(v);
    for elem in v {
        *elem /= m;
    }
}

type Floor = i32;

#[derive(Debug)]
enum Button {
    LobbyCallButton(Floor, Direction),
    CarFloorButton(Floor),
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    DoorOpened,
    DoorClosed,
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCallButton(floor, dir))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloorButton(floor))
}

fn main() {
    // Exercise: Fibonacci
    // ===================
    println!("Exercise: Fibonacci");
    println!("===================");
    println!("fib(20) = {}", fib(20));
    println!();

    // Exercise: Collatz Sequence
    // ==========================
    println!("Exercise: Collatz Sequence");
    println!("==========================");
    println!("collatz_length(11) = {}", collatz_length(11)); // should be 15
    println!();

    // Exercise: Nested Arrays
    // =======================
    println!("Exercise: Nested Arrays");
    println!("=======================");
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
    println!();

    // Exercise: Geometry
    // ==================
    println!("Exercise: Geometry");
    println!("==================");
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
    println!();

    // Exercise: Elevator Events
    // =========================
    println!("Exercise: Elevator Events");
    println!("=========================");
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
