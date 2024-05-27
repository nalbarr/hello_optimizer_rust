// globals
static X_INIT: f64 = 151.0; // initial y guess
static N_EPOCHS: i32 = 100; // number of iterations for algorithm
static ALPHA: f64 = 0.01; // learning rate
static Y_MIN_THRESHOLD: f64 = 0.00001; // early y_min threshold

/*
- original polynomial expression:
10 * (x-2)**2 + 7
"""
"""
- simplify so we can derive gradient later
10 * (x**2 - (4*x)- 4) + 7
(10 * x**2) - (40 * x) - 40 + 7
(10 * x**2) - (40 * x) - 47
*/
fn f(x: f64) -> f64 {
    let base: f64 = x - 2.0;
    let term2: f64 = base.powf(2.0);
    let result = 10.0 * term2 + 7.0;
    result
}

fn find_min<F>(f: F) -> (i32, f64, f64)
where
    F: Fn(f64) -> f64,
{
    fn dump_epoch(y_last: f64, y_new: f64, x_new: f64) {
        fn x_step_str(y_last: f64, y_new: f64) -> &'static str {
            if y_new < y_last {
                "left"
            } else if y_new > y_last {
                "right"
            } else {
                "right"
            }
        }
        println!(
            "y_last: {}, y_new: {}, y_new < y_last: {}, step_dir: {}, x_step: {}",
            y_last,
            y_new,
            (y_new < y_last),
            x_step_str(y_last, y_new),
            x_new
        );
    }

    fn x_step(x: f64) -> f64 {
        let x_new: f64 = x - (ALPHA * gradient(x));
        x_new
    }

    fn y_min_reached(epoch: i32, y_last: f64, y_new: f64, x_new: f64) -> bool {
        if (y_last - y_new).abs() <= Y_MIN_THRESHOLD {
            println!(
                "*** y_min found early epoch {} for: y_new: {}, x_new: {}",
                epoch, y_new, x_new
            );
            true
        } else {
            false
        }
    }

    // initialize to first guess
    let mut x_last = X_INIT;
    let mut y_last = f(X_INIT);
    let mut step_dir = 1.0;
    let mut result: (i32, f64, f64) = (-1, 0.0, 0.0);

    for epoch in 1..N_EPOCHS {
        // compute y_new after x_new step
        let x_new = step_dir * x_step(x_last);
        let y_new = f(x_new);
        dump_epoch(y_last, y_new, x_new);

        if y_new < y_last {
            x_last = x_new;
            y_last = y_new;
            step_dir = 1.0;
        } else if y_new > y_last {
            step_dir = -1.0;
        } else if y_min_reached(epoch, y_last, y_new, x_new) {
            result = (epoch, x_new, y_new);
        }
    }
    result
}

/*
- original polynomial expression
(10 * x**2) - (40 * x) - 47

- find derivative or gradient
(20 * x) - 40 - 0
*/
fn gradient(x: f64) -> f64 {
    (20.0 * x) - 40.0 - 0.0
}

fn main() {
    println!("x_init: {}", X_INIT);
    println!("n_epochs: {}", N_EPOCHS);
    println!("alpha: {}", ALPHA);
    println!("y_min_threshold: {}", Y_MIN_THRESHOLD);

    println!("f(): {}", f(X_INIT));
    let (epochs_actual, x_new, y_min) = find_min(f);

    println!("After: {} epochs", epochs_actual);
    println!("    x_init:  {}", X_INIT);
    println!("    y_init:  {}", f(X_INIT));
    println!("    x_new:   {}", x_new);
    println!("    y_min:   {}", y_min);
}
