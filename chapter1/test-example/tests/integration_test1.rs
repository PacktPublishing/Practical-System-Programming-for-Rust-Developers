use my-process-lib;

    #[test]
    fn test_if_process_id_is_returned() {
        assert_eq!(get_process_id(), 0, "There is error in code");
        println!("Hello from integration test")
    }
