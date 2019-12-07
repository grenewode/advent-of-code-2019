const PROGRAM: &'static [i32] = &[
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 9, 23, 1, 23, 13, 27, 1,
    10, 27, 31, 2, 31, 13, 35, 1, 10, 35, 39, 2, 9, 39, 43, 2, 43, 9, 47, 1, 6, 47, 51, 1, 10, 51,
    55, 2, 55, 13, 59, 1, 59, 10, 63, 2, 63, 13, 67, 2, 67, 9, 71, 1, 6, 71, 75, 2, 75, 9, 79, 1,
    79, 5, 83, 2, 83, 13, 87, 1, 9, 87, 91, 1, 13, 91, 95, 1, 2, 95, 99, 1, 99, 6, 0, 99, 2, 14, 0,
    0,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program: Box<[_]> = Box::from(PROGRAM);

    // Repair the damaged program
    program[1] = 12;
    program[2] = 2;

    let mut vm = intcode::VM::new_debug(&program);

    println!("output part 1 = {} (3085697)", vm.execute()?);

    for i in 0..100 {
        for j in 0..=i {
            let mut program: Box<[_]> = Box::from(PROGRAM);

            // Repair the damaged program
            program[1] = i;
            program[2] = j;

            let mut vm = intcode::VM::new_silent(&program);

            if vm.execute()? == 19690720 {
                println!("output part 2 = {} {}; {}", i, j, 100 * i + j);
            }
        }
    }

    Ok(())
}
