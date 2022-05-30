#[cfg(test)]
mod tests {
    use rust_pipeline_example::Post;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn can_create_a_new_post() {
        let mut post = Post::new();
    }
}
