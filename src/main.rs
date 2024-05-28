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
    let mut n_epochs = 0;
    let mut x_last = X_INIT;
    let mut y_last = f(X_INIT);
    let mut step_dir = 1.0;

    let mut x_new = step_dir * x_step(x_last);
    let mut y_new = f(x_new);

    for epoch in 1..N_EPOCHS {
        dump_epoch(y_last, y_new, x_new);

        if y_new < y_last {
            x_last = x_new;
            y_last = y_new;
            step_dir = 1.0;
        } else if y_new > y_last {
            step_dir = -1.0;
        } else if y_min_reached(epoch, y_last, y_new, x_new) {
            return (n_epochs, x_new, y_new);
        }
        // compute y_new after x_new step
        n_epochs = n_epochs + 1;
        x_new = step_dir * x_step(x_last);
        y_new = f(x_new);
    }
    return (n_epochs, x_new, y_new);
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
    println!("X_INIT: {}", X_INIT);
    println!("N_EPOCHS: {}", N_EPOCHS);
    println!("ALPHA: {}", ALPHA);
    println!("Y_MIN_THRESHOLD: {}", Y_MIN_THRESHOLD);

    println!("f(): {}", f(X_INIT));
    let result: (i32, f64, f64) = find_min(f);

    let epochs_actual: i32 = result.0;
    let x_new: f64 = result.1;
    let y_min: f64 = result.2;

    println!("After: {} epochs", epochs_actual);
    println!("    x_init:  {:.2}", X_INIT);
    println!("    y_init:  {:.2}", f(X_INIT));
    println!("    x_new:   {:.9}", x_new);
    println!("    y_min:   {:.9}", y_min);
}
