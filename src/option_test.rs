#[cfg(test)]
mod tests {
    #[test]
    fn test_copy_trait() {
        let op_int: Option<i32> = Some(1);

        if let Some(mut val1) = op_int {
            val1 = 2;
            println!("Taken value: {val1}");
        }
        
        if op_int.is_some() {
            println!("{:?}", op_int.unwrap());
        }
    }

    #[test]
    fn test_non_copy_trait() {
        let mut op_string = Some("alpha".to_string());

        if let Some(ref mut string_taken) = op_string {
            *string_taken = "beta".to_string();
            println!("taken value: {}", string_taken);
        }

        // This is NOT OK 
        // if op_string.is_some() {
        //     println!("{:?}", op_string.unwrap());
        // }

        println!("----------------------------------");

        let mut op_string_1 = Some("alpha".to_string());

        if let Some(ref mut val) = op_string_1 {
            *val = "beta".to_string();
            println!("taken value of string_1: {}", val);
        }

        if op_string_1.is_some() {
            println!("value of string_1 {:?}", op_string_1.unwrap());
        }

        println!("----------------------------------");

        let mut op_string_2 = Some("aaa".to_string());

        if let Some(val) = op_string_2.as_mut() {
            *val = "bbb".to_string();
            println!("value of string_2: {}", val);
        }

        if op_string_2.is_some() {
            println!("value of string_2 {:?}", op_string_2.unwrap());
        }
    }

    #[test]
    fn test_ref_option() {
        let mut op_string: Option<String> = Some("aaa".to_string());

        let op_string_ref = &mut op_string;

        if let Some(val) = op_string_ref {
            *val = "bbb".to_string();
            println!("value of string: {}", val);
        }

        if let Some(val) = op_string_ref {
            *val = "ccc".to_string();
            println!("value of string: {}", val);
        }
    }

    #[test]
    fn show_stdout() {
        println!("Visible with `cargo test -- --nocapture`.");
    }
}