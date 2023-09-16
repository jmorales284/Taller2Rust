// Don't mind this for now :)
// I AM NOT DONE



#[cfg(test)]
mod tests {
    fn call_me() {
        println!("Ring! Call number ");
    }
    #[test]
    fn call_function() {
        call_me();
    }

}