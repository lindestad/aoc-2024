use p1::*;

#[test]
fn test_solve() {
    let a = Button::new(94, 34, 'A');
    let b = Button::new(22, 67, 'B');
    let p = Prize::new(8400, 5400);

    // A button * 80 presses: 80*3 => 240
    // B button * 40 presses: 40*1 => 40
    // Total: 280
    let expected = 280;
    let result = solve(&a, &b, &p);

    println!("Expected: {}", expected);
    println!("Result: {}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_solve2() {
    let a = Button::new(26, 66, 'A');
    let b = Button::new(67, 21, 'B');
    let p = Prize::new(12748, 12176);

    // No solution exists
    let expected = 0;
    let result = solve(&a, &b, &p);

    println!("Expected: {}", expected);
    println!("Result: {}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_solve3() {
    let input = r"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 200;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_solve4() {
    let input = r"
Button A: X+15, Y+65
Button B: X+99, Y+83
Prize: X=3996, Y=5552
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 160;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_solve5() {
    let input = r"
Button A: X+55, Y+25
Button B: X+23, Y+60
Prize: X=17696, Y=17345
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 0;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_solve6() {
    let input = r"
Button A: X+76, Y+52
Button B: X+14, Y+32
Prize: X=2586, Y=14784
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 0;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);

    assert_eq!(sum, expected);
}

#[test]
fn test_long_input() {
    let input = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 280 + 0 + 200 + 0;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_long_input2() {
    let input = r"
Button A: X+15, Y+26
Button B: X+39, Y+21
Prize: X=1061, Y=6652

Button A: X+79, Y+48
Button B: X+16, Y+66
Prize: X=4606, Y=5106

Button A: X+49, Y+74
Button B: X+89, Y+33
Prize: X=7160, Y=7568

Button A: X+60, Y+14
Button B: X+19, Y+64
Prize: X=9274, Y=1218

Button A: X+19, Y+81
Button B: X+91, Y+22
Prize: X=3752, Y=4651

Button A: X+46, Y+23
Button B: X+16, Y+34
Prize: X=19768, Y=18446

Button A: X+57, Y+21
Button B: X+30, Y+52
Prize: X=2841, Y=3135

Button A: X+63, Y+84
Button B: X+95, Y+38
Prize: X=11781, Y=10122

Button A: X+63, Y+11
Button B: X+23, Y+84
Prize: X=19665, Y=8667

Button A: X+15, Y+41
Button B: X+90, Y+53
Prize: X=3420, Y=2593

Button A: X+92, Y+89
Button B: X+54, Y+11
Prize: X=11260, Y=8336

Button A: X+44, Y+15
Button B: X+13, Y+22
Prize: X=2292, Y=6744

Button A: X+27, Y+68
Button B: X+96, Y+27
Prize: X=8352, Y=8148

Button A: X+81, Y+40
Button B: X+34, Y+76
Prize: X=5787, Y=6588

Button A: X+21, Y+56
Button B: X+43, Y+12
Prize: X=3059, Y=14796

Button A: X+42, Y+41
Button B: X+12, Y+93
Prize: X=1632, Y=9234

Button A: X+54, Y+12
Button B: X+11, Y+56
Prize: X=5853, Y=6408
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 2209;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_long_input3() {
    let input = r"
Button A: X+74, Y+30
Button B: X+22, Y+63
Prize: X=3420, Y=3644

Button A: X+17, Y+57
Button B: X+49, Y+18
Prize: X=3724, Y=15248

Button A: X+83, Y+28
Button B: X+13, Y+67
Prize: X=12829, Y=3710

Button A: X+32, Y+15
Button B: X+13, Y+68
Prize: X=1106, Y=4109

Button A: X+28, Y+89
Button B: X+67, Y+36
Prize: X=2759, Y=2222

Button A: X+50, Y+18
Button B: X+26, Y+44
Prize: X=8434, Y=12054

Button A: X+44, Y+26
Button B: X+13, Y+24
Prize: X=14283, Y=12306

Button A: X+47, Y+29
Button B: X+28, Y+80
Prize: X=1975, Y=3853

Button A: X+75, Y+11
Button B: X+83, Y+74
Prize: X=14876, Y=8179

Button A: X+44, Y+40
Button B: X+95, Y+13
Prize: X=8947, Y=3365

Button A: X+68, Y+11
Button B: X+14, Y+79
Prize: X=10228, Y=13970

Button A: X+45, Y+98
Button B: X+70, Y+31
Prize: X=1410, Y=2342

Button A: X+77, Y+77
Button B: X+75, Y+12
Prize: X=9926, Y=7280

Button A: X+30, Y+56
Button B: X+51, Y+29
Prize: X=4809, Y=6395

Button A: X+95, Y+31
Button B: X+26, Y+94
Prize: X=11225, Y=9649

Button A: X+18, Y+79
Button B: X+86, Y+25
Prize: X=5830, Y=7965

Button A: X+91, Y+43
Button B: X+12, Y+52
Prize: X=4484, Y=3972

Button A: X+15, Y+65
Button B: X+99, Y+83
Prize: X=3996, Y=5552

Button A: X+55, Y+25
Button B: X+23, Y+60
Prize: X=17696, Y=17345

Button A: X+76, Y+52
Button B: X+14, Y+32
Prize: X=2586, Y=14784
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 2578;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}

#[test]
fn test_long_input4() {
    let input = r"


Button A: X+15, Y+65
Button B: X+99, Y+83
Prize: X=3996, Y=5552

Button A: X+55, Y+25
Button B: X+23, Y+60
Prize: X=17696, Y=17345

Button A: X+76, Y+52
Button B: X+14, Y+32
Prize: X=2586, Y=14784
";

    let problems = parse_input(input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    let expected = 160;
    println!("Expected: {}", expected);
    println!("Result: {}", sum);
    assert_eq!(sum, expected);
}
