#[cfg(test)]
mod packet_str_tests {
    use crate::networking::packet_str;
    use std::collections::HashMap;

    #[test]
    fn test_get_from_packet_str() {
        let data_1 = "[ key1 [ key2 [ ] key3 val1 key4 val2 ] key5 val3 key6 [ key7 val4 ] ]"
            .as_bytes()
            .to_vec();
        let graph_1 = packet_str::from_packet(data_1).unwrap();
        let graph_2 = packet_str::ValueType::LIST(HashMap::from([
            (
                "key1".to_owned(),
                packet_str::ValueType::LIST(HashMap::from([
                    (
                        "key2".to_owned(),
                        packet_str::ValueType::LIST(HashMap::new()),
                    ),
                    (
                        "key3".to_owned(),
                        packet_str::ValueType::STRING("val1".to_owned()),
                    ),
                    (
                        "key4".to_owned(),
                        packet_str::ValueType::STRING("val2".to_owned()),
                    ),
                ])),
            ),
            (
                "key5".to_owned(),
                packet_str::ValueType::STRING("val3".to_owned()),
            ),
            (
                "key6".to_owned(),
                packet_str::ValueType::LIST(HashMap::from([(
                    "key7".to_owned(),
                    packet_str::ValueType::STRING("val4".to_owned()),
                )])),
            ),
        ]));
        assert_eq!(graph_1, graph_2)
    }

    #[test]
    fn graph_to_str_to_graph() {
        //we do not check the string as the keys and values could be in any order.
        //Instead we do it both ways. we know string to graph works becoause of the test above, so it's fine
        let graph_1 = packet_str::ValueType::LIST(HashMap::from([
            (
                "key1".to_owned(),
                packet_str::ValueType::LIST(HashMap::from([
                    (
                        "key2".to_owned(),
                        packet_str::ValueType::LIST(HashMap::new()),
                    ),
                    (
                        "key3".to_owned(),
                        packet_str::ValueType::STRING("val1".to_owned()),
                    ),
                    (
                        "key4".to_owned(),
                        packet_str::ValueType::STRING("val2".to_owned()),
                    ),
                ])),
            ),
            (
                "key5".to_owned(),
                packet_str::ValueType::STRING("val3".to_owned()),
            ),
            (
                "key6".to_owned(),
                packet_str::ValueType::LIST(HashMap::from([(
                    "key7".to_owned(),
                    packet_str::ValueType::STRING("val4".to_owned()),
                )])),
            ),
        ]));
        let returned_graph = packet_str::from_packet(packet_str::to_packet(graph_1.clone())).unwrap();
        assert_eq!(graph_1, returned_graph)
    }
}
