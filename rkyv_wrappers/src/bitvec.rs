//! A wrapper that allows to archive a `BitVec` using internally an `ArchivedVec`.

use std::{marker::PhantomData, ops::Deref};

use bitvec::prelude::*;
use rkyv::{
    out_field,
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, Fallible, Serialize,
};

/// A wrapper that allows to archive a `BitVec<T, O>` using internally an `ArchivedVec`.
/// All the `BitSlice<T, O>` methods are available on the archived type thanks to the `Deref` trait implementation.
///
/// Example:
///
/// ```rust
/// use rkyv_wrappers::bitvec::BitVecWrapper;
/// use bitvec::prelude::*;
/// use rkyv::{
///     archived_root,
///     ser::{serializers::AllocSerializer, Serializer},
///     Archive, Deserialize, Infallible, Serialize,
/// };
///
/// #[derive(Archive, Serialize, Deserialize, PartialEq, Debug)]
/// struct StructWithBitVec {
///     #[with(BitVecWrapper)]
///     pub bitvec: BitVec,
/// }
///
/// let mut serializer = AllocSerializer::<4096>::default();
/// let original = StructWithBitVec {
///     bitvec: bitvec![1, 0, 1, 1, 1],
/// };
/// serializer.serialize_value(&original).unwrap();
/// let buffer = serializer.into_serializer().into_inner();
///
/// let output = unsafe { archived_root::<StructWithBitVec>(&buffer) };
/// assert_eq!(&original.bitvec, output.bitvec.as_slice());
///
/// let deserialized: StructWithBitVec = output.deserialize(&mut Infallible).unwrap();
/// assert_eq!(deserialized, original);
/// ```
pub struct BitVecWrapper;

/// An archived `BitVec`.
// We also have to store the bit length.
// This is because when calling `as_raw_slice` we will get unwanted bits if the `BitVec` bit length is not a multiple of the bit size of T.
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
pub struct ArchivedBitVec<T: BitStore + Archive, O: BitOrder> {
    inner: ArchivedVec<Archived<T>>,
    bit_len: Archived<usize>,
    _or: PhantomData<O>,
}

impl<T: BitStore + Archive, O: BitOrder> ArchivedBitVec<T, O>
where
    Archived<T>: BitStore,
{
    /// Gets the elements of the archived `BitVec` as a `BitSlice`.
    pub fn as_slice(&self) -> &BitSlice<Archived<T>, O> {
        self.deref()
    }
}

impl<T: BitStore + Archive, O: BitOrder> Deref for ArchivedBitVec<T, O>
where
    Archived<T>: BitStore,
{
    type Target = BitSlice<Archived<T>, O>;

    fn deref(&self) -> &Self::Target {
        &self.inner.view_bits::<O>()[..self.bit_len as usize]
    }
}

impl<T: BitStore + Archive, O: BitOrder> ArchiveWith<BitVec<T, O>> for BitVecWrapper {
    type Archived = ArchivedBitVec<T, O>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &BitVec<T, O>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let (fp, fo) = out_field!(out.inner);
        ArchivedVec::resolve_from_slice(field.as_raw_slice(), pos + fp, resolver, fo);
        let (fp, fo) = out_field!(out.bit_len);
        usize::resolve(&field.len(), pos + fp, (), fo);
    }
}

impl<
        T: BitStore + Archive + Serialize<S>,
        O: BitOrder,
        S: Fallible + ?Sized + ScratchSpace + Serializer,
    > SerializeWith<BitVec<T, O>, S> for BitVecWrapper
{
    fn serialize_with(
        field: &BitVec<T, O>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        let resolver = ArchivedVec::serialize_from_slice(field.as_raw_slice(), serializer)?;
        usize::serialize(&field.len(), serializer)?;

        Ok(resolver)
    }
}

impl<T: BitStore + Archive, O: BitOrder, D: Fallible + ?Sized>
    DeserializeWith<ArchivedBitVec<T, O>, BitVec<T, O>, D> for BitVecWrapper
where
    Archived<T>: Deserialize<T, D>,
{
    fn deserialize_with(
        field: &ArchivedBitVec<T, O>,
        deserializer: &mut D,
    ) -> Result<BitVec<T, O>, <D as Fallible>::Error> {
        let vec = ArchivedVec::deserialize(&field.inner, deserializer)?;
        let bit_len = Archived::<usize>::deserialize(&field.bit_len, deserializer)?;

        let mut bitvec = BitVec::<T, O>::from_vec(vec);
        bitvec.truncate(bit_len);
        Ok(bitvec)
    }
}
