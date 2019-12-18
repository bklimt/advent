
use std::env;
use std::fs;

fn phase(input: &Vec<i32>, output: &mut Vec<i32>, n: usize) {
    for i in 0..n {
        output[i] = 0;
        let mut s = 1;
        let mut j = i;
        while j < n {
            // Add the ones that are non-zero.
            let mut end = j+i+1;
            if end > n {
                end = n;
            }
            for k in j..end {
                output[i] += s * input[k];
            }
            // Skip the ones that are zero.
            j = end+i+1;
            s = s*-1;
        }
        output[i] = output[i].abs() % 10;
        //print!("{}", ((output[i] as u8) + ('0' as u8)) as char);
    }
    //println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let bytes = fs::read(filename)
        .expect("unable to read file");

    let m = bytes.len();
    let n = m * 10000;
    println!("n = {}", n);

    let mut buf1 = vec![0; n];
    let mut buf2 = vec![0; n];

    // Copy the input into a buffer.
    for i in 0..n {
        buf1[i] = (bytes[i % m] as i32) - ('0' as i32);
    }

    // Run the loop several times.
    for i in 0..50 {
        println!("Phase {}", i*2);
        phase(&buf1, &mut buf2, n);
        println!("Phase {}", i*2+1);
        phase(&buf2, &mut buf1, n);
    }

    // Print the result.
    for i in 0..n {
        print!("{}", ((buf1[i] as u8) + ('0' as u8)) as char);
    }
    println!();
}