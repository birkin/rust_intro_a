fn main() {

    println!("\n---\nabs()...\n---");
    let val = 2.0;
    let res = abs( val );
    println!("absolute-value of {} is {}", val, res );
    let val = -2.0;
    let res = abs( val );
    println!("absolute-value of {} is {}", val, res );

    println!("\n---\nclamp()...\n---");
    let v = 2.0; let v1 = 3.0; let v2 = 4.0;
    let res = clamp( v, v1, v2 );
    println!("middle value of {}, {}, and {} is {}", v, v1, v2, res );

    println!("\n---\nfactorial()...\n---");
    let val = 10;
    let res = factorial( val );
    println!( "value, {}; result, {}", val, res );

    println!("\n---\nby_ref()...\n---");
    let val = 10;
    let res = by_ref( &val );
    println!( "value, {}; result, {}", val, res );

    println!("\n---\nmodifies()...\n---");
    let mut val = 10.0;
    let res = modifies( &mut val );
    println!( "value, {:?}; result, {:?}", val, res );
}

// modifying reference
fn modifies( x: &mut f64 ) {
    println!( "x initially, {:?}", x );
    // *x = 1.0
    let res = *x = 1.0;
    println!( "res, {:?}", res );
    return res;
}

// values passed by reference
fn by_ref( x: &i32 ) -> i32 {
    *x + 1
}

// factorial
fn factorial( n: u64 ) -> u64 {
    // let mut res: u64 = 0;
    if n == 0 {
        println!( "n must be zero" );
        let res: u64 = 1;
        return res
    } else {
        println!( "n before recursion-call, {}", n );
        let res: u64 = n * factorial( n-1 );
        println!( "n currently, {}; res currently, {}", n, res );
        // return res;
        res
    }
}

// absolute value of a floating-point number
fn abs( x: f64 ) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp( x: f64, x1: f64, x2: f64 ) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}
