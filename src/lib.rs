pub mod helper_old;

pub mod opengl_renderer;

pub mod vulkan_renderer;

mod glfw3_helper;

#[cfg(test)]
mod tests {
    use helper_old;
    use opengl_renderer;
    use vulkan_renderer;

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
    fn test_opengl_then_vulkan_main() {
        opengl_renderer::main();
        vulkan_renderer::main();
    }

}