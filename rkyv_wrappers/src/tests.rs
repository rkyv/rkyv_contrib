pub mod as_hashmap {
    #[test]
    fn sruct_with_hashmap() {
        use rkyv::{
            archived_root,
            ser::{serializers::AlignedSerializer, Serializer},
            AlignedVec, Deserialize, Infallible,
        };
        #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug, PartialEq, Eq)]
        struct StructWithHashMap {
            #[with(crate::as_hashmap::AsHashMap)]
            pub hash_map: Vec<(u32, String)>,
        }
        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        let original = StructWithHashMap {
            hash_map: vec![(1, String::from("a")), (2, String::from("b"))],
        };
        serializer.serialize_value(&original).unwrap();
        let buffer = serializer.into_inner();
        let output = unsafe { archived_root::<StructWithHashMap>(&buffer) };
        assert_eq!(output.hash_map.get(&1).unwrap(), &"a");
        let deserialized: StructWithHashMap = output.deserialize(&mut Infallible).unwrap();
        assert_eq!(deserialized, original);
    }
}
