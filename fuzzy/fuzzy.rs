#![allow(unused)]
#![allow(dead_code)]

use std::io::{self, Write}; 

fn fuzzification(x: f32, a: f32, b: f32, c: f32) -> (f32, f32, f32) {
    let mut u1: f32 = 0.0;
    let mut u2: f32 = 0.0;
    let mut u3: f32 = 0.0;

    let mut u_left: f32 = 0.0;
    let mut u_center: f32 = 0.0;
    let mut u_right: f32 = 0.0;

    let left_c: f32 = a;
    let left_d: f32 = b;
    let ctr_a: f32 = a;
    let ctr_b: f32 = b;
    let ctr_c: f32 = c;
    let right_a: f32 = b;
    let right_b: f32 = c;

    // left trapezoid // sebelah kiri trapezoid
    if x <= left_c {
        u_left = 1.0;
    } else if x > left_c && x < left_d {
        u_left = (left_d - x) / (left_d - left_c);
    } else if x >= left_d {
        u_left = 0.0;
    }
    // println!("u_left: {}", u_left); // to be replaced

    u1 = u_left;

    // ceter triangle // segitiga ditengah

    if x <= ctr_a || x >= ctr_c {
        u_center = 0.0;
    } else if x > ctr_a && x < ctr_b {
        u_center = (x - ctr_a) / (ctr_b - ctr_a);
    } else if x == ctr_b {
        u_center = 1.0;
    } else if x > ctr_b && x < ctr_c {
        u_center = (ctr_c - x) / (ctr_c - ctr_b);
    }
    // println!("u_center: {}", u_center)

    u2 = u_center;

    // right trapezoid // sebelah kanan trapezoid
    if x <= right_a {
        u_right = 0.0;
    } else if x > right_a && x < right_b {
        u_right = (x - right_a) / (right_b - right_a);
    } else if x >= right_b {
        u_right = 1.0;
    }
    // println!("u_right: {}", u_right);

    u3 = u_right;

    (u1, u2, u3)
}

fn and_rules(f1: f32, f2: f32, temp: &mut f32, conclusion: &mut f32) {
    if f1 >= f2 {
        *temp = f2;
    } else {
        *temp = f1;
    }
    if *temp >= *conclusion {
        *conclusion = *temp;
    }
}

fn determine_sample(a: f32, b: f32, c: f32, d: f32) -> (f32, f32, f32) {
    let s1: f32 = (a + b) / 2.0;
    let s2: f32 = (b + c) / 2.0;
    let s3: f32 = (c + d) / 2.0;
    (s1, s2, s3)
}

fn defuzzification(u1: f32, u2: f32, u3: f32, s1: f32, s2: f32, s3: f32) -> f32 {
    ((u1 * s1) + (u2 * s2) + (u3 * s3)) / (u1 + u2 + u3)
}

fn main() {
    let x_explv: f32;
    let x_exptm: f32;

    let mut u_low: f32 = 0.0;
    let mut u_med: f32 = 0.0;
    let mut u_high: f32 = 0.0;

    let mut u_fast: f32 = 0.0;
    let mut u_norm: f32 = 0.0;
    let mut u_slow: f32 = 0.0;

    let mut poor: f32 = 0.0;
    let mut average: f32 = 0.0;
    let mut awesome: f32 = 0.0;

    let mut first_sample: f32 = 0.0;
    let mut second_sample: f32 = 0.0;
    let mut third_sample: f32 = 0.0;

    let mut reward: f32 = 0.0;

    print!("Masukkan x untuk expl level : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    x_explv = input.trim().parse().unwrap();

    print!("Masukkan x untuk expl time : ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    x_exptm = input.trim().parse().unwrap();

    let (u1, u2, u3) = fuzzification(x_explv, 40.0, 60.0, 80.0); // exploration level
    let (u4, u5, u6) = fuzzification(x_exptm, 15.0, 30.0, 45.0); // exploration time

    u_low = u1;
    u_med = u2;
    u_high = u3;
    u_fast = u4;
    u_norm = u5;
    u_slow = u6;

    println!("Fuzzification of exploration level : ");
    println!("u_low = {:.2}", u_low);
    println!("u_med = {:.2}", u_med);
    println!("u_high = {:.2}", u_high);

    println!("Fuzzification of exploration time : ");
    println!("u_fast = {:.2}", u_fast);
    println!("u_norm = {:.2}", u_norm);
    println!("u_slow = {:.2}", u_slow);

    let mut temp_poor = 0.0;
    let mut temp_average = 0.0;
    let mut temp_awesome = 0.0;

    and_rules(u_low, u_low, &mut temp_poor, &mut poor);
    and_rules(u_low, u_norm, &mut temp_poor, &mut poor);
    and_rules(u_low, u_slow, &mut temp_average, &mut average);
    and_rules(u_med, u_low, &mut temp_average, &mut average);
    and_rules(u_med, u_norm, &mut temp_average, &mut average);
    and_rules(u_med, u_slow, &mut temp_awesome, &mut awesome);
    and_rules(u_high, u_low, &mut temp_average, &mut average);
    and_rules(u_high, u_norm, &mut temp_awesome, &mut awesome);
    and_rules(u_high, u_slow, &mut temp_awesome, &mut awesome);
    
    println!("\nConclusion : ");
    println!("u_Poor = {:.2}", poor);
    println!("u_Average = {:.2}", average);
    println!("u_Awesome = {:.2}", awesome);

    // sample for defuzzification
    let (first_sample, second_sample, third_sample) = determine_sample(0.0, 40.0, 80.0, 100.0);
    println!("\nSample value for defuzzification : ");
    println!("Sample 1 = {:.2}", first_sample);
    println!("Sample 2 = {:.2}", second_sample);
    println!("Sample 3 = {:.2}", third_sample);

    // defuzzification
    reward = defuzzification(poor, average, awesome, first_sample, second_sample, third_sample);
    println!("\n\nREWARD :");
    println!("{:.2}", reward);
}
