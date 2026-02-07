//cargo test check_memory_leaks -- --nocapture

#[cfg(test)]
mod memory_leak_test {
    use crate::MyEguiApp;

    #[test]
    fn check_memory_leaks() {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        {
            let app = MyEguiApp::new_for_testing();

            //Maybe run integration tests

            drop(app);
        }
    }
}
