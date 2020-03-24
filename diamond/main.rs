// Problem Description: http://codingdojo.org/kata/Diamond/
// Given a letter, print a diamond starting with ‘A’ with the supplied letter at the widest point.
// For example: print-diamond ‘C’ prints
//     A
//    B B
//   C   C
//  D     D
// E       E
//  D     D
//   C   C
//    B B
//     A

fn main() {
	diamond('L');
}

// input: char
// pisos = ascii(input) - 66 (ascii(A)+1)
// for i..pisos
// for
fn diamond(middle: char) {
    let steps = (middle as u32 - 65) + 1;
    let mut n_space = 1;
   
    if steps == 1 {
       println!("{}", 'A');
       return
    }
    let first_padding = " ".repeat(steps as usize);
    println!("{}{}", first_padding, 'A'); // TODO: calculate left padding
    
    for step in  1..steps {
        let letter: char = std::char::from_u32(65 + step).unwrap();
        let space = " ".repeat(n_space);
        let n = (steps - step) as usize;
        let lpadding = " ".repeat(n);
        println!( "{}{}{}{}", lpadding, letter, space, letter);
        n_space += 2;
    }
    n_space -= 2;
    for step in (1..steps-1).rev() {
        let letter: char = std::char::from_u32(65 + step).unwrap();
        n_space -= 2;
        let space = " ".repeat(n_space);
        let n = (steps - step) as usize;
        let lpadding = " ".repeat(n);
        println!( "{}{}{}{}", lpadding, letter, space, letter);
    }
    println!("{}{}", first_padding, 'A'); // TODO: calculate left padding
}

