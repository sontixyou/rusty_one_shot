pub mod another_number_calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub mod nested_another_number_calculator {
        pub fn sub(a: i32, b: i32) -> i32 {
            a - b
        }
    }
}
