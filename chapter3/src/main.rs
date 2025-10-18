
// // Fix the variable definition of 'x'

// fn main() {
//     let x = 5;
//     println!("x has the value {}", x);
//     let x="abc";
//     println!("new x has the val {}",x);

// }


// Something's missing. Fix the code so that it compiles.

// fn main() {
//     let is_morning = true;
//     if is_morning {
//         println!("Good morning!");
//     }

//     let is_evening=true;  // Finish the rest of this line
//     if is_evening {
//         println!("Good evening!");
//     }
// }

// Make this program compile by replacing the variable type.

// fn main() {
//     let number_of_stars: i64;
//     // The Milky Way has more stars than can fit in a 32-bit integer type!
//     number_of_stars = 400_000_000_000;
//     println!("There are about {} stars in the Milky Way galaxy!", number_of_stars);
// }


// Make this program compile without changing the variable type!

// fn main() {
//     let answer: String ="green".to_string();
//     println!("My current favorite color is {}", answer);
// }

// Create an array with at least 10 elements in it.

// fn main() {
//     let a = [2;15] /* Your array here */;

//     if a.len() >= 10 {
//         println!("Wow, that's a big array!");
//     } 
// }


// Destructure the `cat` tuple so that the println will work.

// fn main() {
//     let cat = ("Furry McFurson", 3.5);
//     let (name,age) = cat;

//     println!("{} is {} years old.", name, age);
// }


// Add a type alias for our dogs so we can store their names and ages.

fn main() {
    type Dog = (String,u8);

    let dog1: Dog = (String::from("Albert"), 3);
    println!("My dog {} is {} years old.", dog1.0, dog1.1);

    let dog2: Dog = (String::from("Barker"), 5);
    println!("My other dog {} is {} years old.", dog2.0, dog2.1);
}