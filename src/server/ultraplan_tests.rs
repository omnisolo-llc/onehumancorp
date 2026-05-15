use super::*;

    #[test]
    fn test_compress_decompress() {
        let data = b"hello world hello world hello world";
        let compressed = compress_ultraplan_data(data).unwrap();
        let decompressed = decompress_ultraplan_data(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_create_plan() {
        let manager = UltraPlanManager::new();
        let state_machine = serde_json::json!({"phase": "INIT"});
        let plan = manager.create_plan("mission1".to_string(), state_machine.clone()).unwrap();

        assert_eq!(plan.mission_id, "mission1");
        assert_eq!(plan.status, "DELIBERATING");
        assert_eq!(plan.state_machine, state_machine);

        let fetched = manager.get_ultra_plan(&plan.id).unwrap();
        assert_eq!(fetched.id, plan.id);
    }
