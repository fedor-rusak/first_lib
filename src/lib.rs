pub mod helper_old;

pub mod glfw3_helper;


#[cfg(test)]
mod tests {
    use helper_old;
    use glfw3_helper;

    #[test]
    fn it_works() {
        const CORRECT_ANSWER: i32 = 42;
        let answer = helper_old::answer_for_everything();
        assert_eq!(
            answer,
            CORRECT_ANSWER,
            "{} is not an answer for everything please refer to hitchhikers guide!",
            answer);
    }

    #[test]
    fn maybe_main() {
        glfw3_helper::main();
    }
}