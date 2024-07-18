#[allow(unused_variables, dead_code)]
mod outer {
    pub mod inner {
        pub fn public_function() {
            println!("This is a public function");
        }

        fn private_function() {
            println!("This is a private function");
        }
    }

    fn outer_private_function() {
        println!("This is a private function in the outer module");
    }
}

fn main() {
    outer::inner::public_function(); // Works
//     outer::inner::private_function(); // Error: function `private_function` is private
//     outer::outer_private_function(); // Error: function `outer_private_function` is private
}
