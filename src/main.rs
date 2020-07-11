use proptest::prelude::*;

#[derive(Debug)]
struct U24(u32);

impl Arbitrary for U24 {
    type Parameters = ();

    fn arbitrary_with(_args: ()) -> Self::Strategy {
        prop_oneof![(0..(1_u32 << 24)).prop_map(U24),].boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}

fn main() {}
