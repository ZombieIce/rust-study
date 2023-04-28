pub mod animal;

#[cfg(test)]
mod tests {
    use crate::animal;

    #[test]
    fn use_cat() {
        animal::cat::hello();
        assert!(animal::cat::is_cat());
    }

    #[test]
    fn use_dog() {
        assert!(animal::dog::is_dog());
    }
}