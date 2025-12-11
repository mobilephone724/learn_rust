#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_trait() {
        let op_copy: Option<i32> = Some(1);

        if let Some(mut val1) = op_copy {
            val1 = 2;
            println!("Taken value: {val1}");
        } else {
            println!("op_copy was None");
        }

        if op_copy.is_some() {
            let val2 = op_copy.unwrap();
            println!("Taken value: {val2}");
        }

        if let Some(val3) = op_copy {
            println!("Taken value: {val3}");
        } else {
            println!("op_copy was None");
        }
    }

    #[test]
    fn test_non_copy_trait() {
        struct NonCopy {
            data: String,
        }

        let op_non_copy = Some(NonCopy {
            data: "alpha".to_string(),
        });

        if let Some(mut borrowed) = op_non_copy {
            borrowed.data = "beta".to_string();
            println!("Borrowed value: {}", borrowed.data);
        }

        // This is NOT OK 
        // println!("{:?}", op_non_copy.unwrap().data);
    }

    #[test]
    fn show_stdout() {
        println!("Visible with `cargo test -- --nocapture`.");
    }
}