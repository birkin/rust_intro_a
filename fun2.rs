fn main() {
    fn sqr( x: f64 ) -> f64 {
        x * x
    }
    let res = sqr( 2.0 );
    println!("square is {}", res );
}
