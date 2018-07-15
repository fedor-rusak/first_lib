pub mod helper;


#[cfg(test)]
mod tests {
    use helper;

    #[test]
    fn it_works() {
        const CORRECT_ANSWER: i32 = 42;
        let answer = helper::answer_for_everything();
        assert_eq!(
            answer,
            CORRECT_ANSWER,
            "{} is not an answer for everything please refer to hitchhikers guide!",
            answer);
    }
}