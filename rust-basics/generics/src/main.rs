#[derive(Debug)]
struct Point<T, V> {
    x: T,
    y: V,
}

impl<T, V> Point<T, V> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 100, 65, 25];
    let char_list = vec!['a', 'b', 'c', 'd'];

    let largest = get_largest(number_list);
    let largest_char = get_largest(char_list);

    println!("The largest number in the array is {largest}");
    println!("The largest number in the array is {largest_char}");

    let p1 = Point { x: 5, y: 10.0 };
    let p2 = Point { x: 'c', y: 1 };

    println!("{:#?}", p1);
    println!("{:#?}", p2);
}

fn get_largest<T: PartialOrd + Copy>(numbers: Vec<T>) -> T {
    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    largest
}
