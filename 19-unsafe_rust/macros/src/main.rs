#[macro_export]
macro_rules! vec_t {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec_t![1, 2, 3];

    for i in &v {
        print!("{} ", i);
    }
}
