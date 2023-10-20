fn main() {
    // let x = 12;
    // let a = 12u8;
    // let b = 4.3;
    // let c = 4.3f32;
    // let bv = true;
    // let t = (13, false);
    // let sentence = "witaj swiecie";

    let x = 13;
    println!("{}", x);
    println!("{x}");

    let x: f64 = 3.14159;
    println!("{x}");

    let x;
    x = 0;
    println!("{x}");

    let mut x = 42;

    println!("{x}");
    x = 4534;

    println!("{x}");

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{c}");

    let t = true;
    println!("{}", t as u8);

    zadanie1();
    zadanie1_easy();
    zadanie2();

    zadanie3();

    zadanie4();

    zadanie5();

    zadanie6();

    let mut x = 0;

    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }

    println!("{}", x);

    zadanie7();

    zadanie8();

    let x = 7;

    match x {
        0 => {
            println!("dostalismy 0");
        }
        1 | 2 => {
            println!("dostalismy 1 lub 2");
        }
        3..=9 => {
            println!("between 3 and 9 (inclusive left and right)");
        }

        matched_num @ 10..=100 => {
            println!("between 10 and 100 (inclusive left): {matched_num}");
        }

        _ => {
            println!("something different");
        }
    }

    zadanie9();
    zadanie10();
}

fn zadanie10(){
    println!("zadanie10");
    let speed = 3e4;//30.0 * 1000.0; // m/s
    let time_period_hour = 3.6e3; // 60.0 * 60.0; // in seconds
    let time_period_day = time_period_hour * 2.4e1; //24.0; 
    let time_period_year = time_period_day * 3.6525e2; //365.25;

    let distance_in_hour = speed * time_period_hour / 1000.0;
    let distance_in_day = speed * time_period_day  / 1000.0;
    let distance_in_year = speed * time_period_year  / 1000.0;

    println!("distance_in_hour: {distance_in_hour} km.");
    println!("distance_in_day: {distance_in_day} km.");
    println!("distance_in_year: {distance_in_year} km.");
}

fn zadanie9() {
    println!("zadanie9");
    let a = 1.2345e5;
    println!("a: {a}");
    let b = 0.12345e0;
    println!("b: {b}");
    let c = 0.123e-5;
    println!("c: {c}");
    let d = 1.02030405e8;
    println!("d: {d}");

}

fn zadanie8() {
    println!("zadanie8");

    /// k0: initial investment, for example 1000
    /// r: yearly interest rate, for example 0.05
    /// m: how many capitalizations in a year, for example 4
    /// n: period in years, for example 2
    fn compound_interest(k0: f64, r: f64, m: f64, n: f64) -> f64{
        return k0 * ((1.0 + r / m) as f64).powf(m * n);
    }
    
    let k0 = 1000.0;
    let r = 0.05;
    let m = 1.0;
    let n = 2.0;

    let k: f64 = compound_interest(k0, r, m, n);
    println!("for parameters: initial investment {k0}, interest rate is {r}%, capitalization {m} and period is {n} years");
    println!("final capital: {k}");


    let k0 = 1000.0;
    let r = 0.023;
    let m = 4.0;
    let n = 3.0;

    let k: f64 = compound_interest(k0, r, m, n);
    println!("for parameters: initial investment {k0}, interest rate is {r}%, capitalization {m} and period is {n} years");
    println!("final capital: {k}");

}

fn zadanie7() {
    println!("zadanie7");
    let h0 = 12.5; // in meters
    let v0 = 3.0; // m/s
    let alpha = 0.12; // in radians

    fn calculate_height(h0: f64, g: f64, alpha: f64, v0: f64, x: f64) -> f64 {
        return h0 + x * alpha.tan() - (g * x * x) / (2.0 * v0 * v0 * alpha.cos().powf(2.0));
    }

    let x = 2.963; // in meters
    let g = 9.8; // in meters / s^2
    let height = calculate_height(h0, g, alpha, v0, x);
    println!(
        "Height is {height} for distance x = {x}, initial height: {h0}, initial velocity: {v0}, "
    )
}

fn zadanie6() {
    println!("zadanie6");

    let r = 10.0;
    let angle_in_degrees: f64 = 30.0;

    let area = std::f64::consts::PI * r * r * (angle_in_degrees / 360.0);
    let basic_circumference = std::f64::consts::PI * 2.0 * r * (angle_in_degrees / 360.0);
    // when the circle is not full then add radius twice;
    let circumference = if (angle_in_degrees - 360.0).abs() < 0.0005 {
        basic_circumference
    } else {
        basic_circumference + 2.0 * r
    };

    println!("circle with radius: {r}, has area: {area}, and circumference: {circumference}.");
}

fn zadanie5() {
    println!("zadanie5");

    let r = 10.0;

    let area = std::f64::consts::PI * r * r;
    let circumference = std::f64::consts::PI * 2.0 * r;
    println!("circle with radius: {r}, has area: {area}, and circumference: {circumference}.");
}

fn zadanie4() {
    println!("zadanie4");
    let sample_temperature_in_c = 36.6;

    let sample_temperature_in_f = convert_temperature_c_to_f(sample_temperature_in_c);
    println!("{sample_temperature_in_c} in C = {sample_temperature_in_f} in F.");

    let verify_sample_temperature_in_c = convert_temperature_f_toc(sample_temperature_in_f);
    let diff = sample_temperature_in_c - verify_sample_temperature_in_c;
    println!("diff is {}", diff.abs());
}

fn zadanie3() {
    println!("zadanie 3");
    let meters = 640.0;
    let cals = convert_meter_to_cal(meters);
    let feet = convert_meter_to_foot(meters);
    let yard = convert_meter_to_yard(meters);
    let miles = convert_meter_to_mile(meters);

    println!(
        "{meters} meters = {cals} cals = {feet} \
    feet = {yard} yard = {miles} miles."
    );
}

fn zadanie2() {
    let v0 = 3.44; // m/s
    let times: [f64; 3] = [0.54, 0.1, 0.235];
    let gravity_on_planets = [
        ("sun", 293.0),
        ("mercury", 3.7),
        ("venus", 8.8),
        ("earth", 9.8),
        ("moon", 1.7),
        ("mars", 3.7),
        ("ceres", 0.27),
        ("jupiter", 24.7),
        ("saturn", 10.5),
        ("uranus", 9.0),
        ("neptune", 11.7),
        ("pluto", 0.49),
    ];

    gravity_on_planets.into_iter().for_each(|(planet_name, planet_g)| {
        times.into_iter().for_each( |time: f64|  {
        
            let height: f64 = calculate_height(v0, planet_g, time);
    
            println!("On planet {planet_name}, with acceleration: {planet_g} m/s^2, height after time: {time} = {height}.");
        });
        println!();
    })
}

fn zadanie1_easy() {
    let v0: f64 = 3.44;
    let g: f64 = 9.81;

    println!("predkosc poczatkowa: {v0}.");
    println!("przyspieszenie grawitacyjne: {g}.");
    let time0_0: f64 = 0.54;
    let time0_1: f64 = 0.1;
    let time0_2: f64 = 0.235;

    println!(
        "Dla czasu lotu: {}, wysokosc wynosi: {}",
        time0_0,
        calculate_height(v0, g, time0_0)
    );
    println!(
        "Dla czasu lotu: {}, wysokosc wynosi: {}",
        time0_0,
        calculate_height(v0, g, time0_1)
    );
    println!(
        "Dla czasu lotu: {}, wysokosc wynosi: {}",
        time0_0,
        calculate_height(v0, g, time0_2)
    );
}

fn zadanie1() {
    //
    println!("zadanie 1");
    // find heights
    let v0 = 3.44; // m/s
    let g = 9.8; // m/s^2

    let times: [f64; 3] = [0.54, 0.1, 0.235];
    times.into_iter().for_each(|time: f64| {
        let height: f64 = calculate_height(v0, g, time);

        println!("height after time: {time} = {height}.");
    });
}

fn convert_temperature_c_to_f(c: f64) -> f64 {
    return 9.0 / 5.0 * c + 32.0;
}

fn convert_temperature_f_toc(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

fn calculate_height(v0: f64, g: f64, t: f64) -> f64 {
    return v0 * t - 0.5 * g * t * t;
}

fn convert_meter_to_cal(meters: f64) -> f64 {
    return cm_to_cal(meters * 100.0);
}

fn convert_meter_to_foot(meters: f64) -> f64 {
    return convert_meter_to_cal(meters) / 12.0;
}

fn convert_meter_to_yard(meters: f64) -> f64 {
    return convert_meter_to_foot(meters) / 3.0;
}

fn convert_meter_to_mile(meters: f64) -> f64 {
    return convert_meter_to_yard(meters) / 1760.0;
}

// fn cal_to_cm(cal: f64)-> f64{
//     return cal * 2.54;
// }

fn cm_to_cal(cm: f64) -> f64 {
    return cm / 2.54;
}
