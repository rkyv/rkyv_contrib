use rkyv::{
    ser::Serializer,
    std_impl::chd::{ArchivedHashMap, ArchivedHashMapResolver},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Deserialize, Fallible, Serialize,
};
use std::{hash::Hash, mem::MaybeUninit};
/// A wrapper that attempts to convert a vector to and from `ArchivedHashMap`
///
/// rkyv's `ArchivedHashMap` uses a fairly different implementation than `HashMap` in the standard
/// library. Therefore, constructing `HashMap` and converting it to `ArchivedHashMap` will create
/// unnecessary hashes that will never be used. By labeling a vector `AsHashMap`, you can use its
/// archived version just like `ArchivedHashMap` without having costy `HashMap` creations.
///
/// Example:
///
/// ```rust
/// # use rkyv::{AlignedVec, Deserialize, Infallible, archived_root, ser::{Serializer, serializers::AlignedSerializer}};
/// #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug, PartialEq, Eq)]
/// struct StructWithHashMap {
///     #[with(rkyv_wrappers::as_hashmap::AsHashMap)]
///     pub hash_map: Vec<(u32, String)>,
/// }
/// let mut serializer = AlignedSerializer::new(AlignedVec::new());
/// let original = StructWithHashMap {
///     hash_map: vec![(1, String::from("a")), (2, String::from("b"))]
/// };
/// serializer.serialize_value(&original).unwrap();
/// let buffer = serializer.into_inner();
/// let output = unsafe {
///     archived_root::<StructWithHashMap>(&buffer)
/// };
/// assert_eq!(output.hash_map.get(&1).unwrap(), &"a");
/// let deserialized: StructWithHashMap = output.deserialize(&mut Infallible).unwrap();
/// assert_eq!(deserialized, original);
/// ```
pub struct AsHashMap;
impl<K: Archive, V: Archive> ArchiveWith<Vec<(K, V)>> for AsHashMap {
    type Archived = ArchivedHashMap<K::Archived, V::Archived>;
    type Resolver = ArchivedHashMapResolver;
    #[inline]
    fn resolve_with(
        field: &Vec<(K, V)>,
        pos: usize,
        resolver: Self::Resolver,
        out: &mut MaybeUninit<Self::Archived>,
    ) {
        resolver.resolve_from_len(pos, field.len(), out);
    }
}
impl<
        K: Archive + Serialize<S> + Hash + Eq,
        V: Archive + Serialize<S>,
        S: Serializer + Fallible + ?Sized,
    > SerializeWith<Vec<(K, V)>, S> for AsHashMap
{
    #[inline]
    fn serialize_with(field: &Vec<(K, V)>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedHashMap::serialize_from_iter(
            field.iter().map(|(x, y)| (x, y)),
            field.len(),
            serializer,
        )
    }
}
impl<K: Archive, V: Archive, D: Fallible + ?Sized>
    DeserializeWith<ArchivedHashMap<K::Archived, V::Archived>, Vec<(K, V)>, D> for AsHashMap
where
    K::Archived: Deserialize<K, D>,
    V::Archived: Deserialize<V, D>,
{
    #[inline]
    fn deserialize_with(
        field: &ArchivedHashMap<K::Archived, V::Archived>,
        deserializer: &mut D,
    ) -> Result<Vec<(K, V)>, D::Error> {
        field
            .iter()
            .map(|(k, v)| Ok((k.deserialize(deserializer)?, v.deserialize(deserializer)?)))
            .collect()
    }
}
