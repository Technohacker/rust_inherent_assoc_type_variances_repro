pub struct Struct1;
pub struct Struct2;
pub struct Struct3;

// These two from external crate
pub trait ExternalTrait {
    type Output;

    fn do_something() -> Self::Output;
}

pub trait ExternalUtilityTrait<T> {
    type Output;

    fn do_something_with_type(&mut self, input: T) -> Self::Output;
}

// These from current crate
// ########## Variances Cycle Error raised here ########
pub struct ProblemStruct<T: HelperTrait> {
    inner: T,
}

pub trait HelperTrait: ExternalUtilityTrait<Struct2> {}

impl<T: HelperTrait> ExternalTrait for ProblemStruct<T> {
    type Output = ();

    fn do_something() -> Self::Output {
        todo!()
    }
}

impl<T: HelperTrait> ExternalUtilityTrait<Struct1> for ProblemStruct<T> {
    type Output = ();

    // Commenting this function out correctly shows the
    // unstable inherent assoc. types warning
    fn do_something_with_type(&mut self, input: Struct1) -> Self::Output {
        todo!()
    }
}

impl<T: HelperTrait> ProblemStruct<T> {
    // Inherent associated type used here
    type Output = ();
}
fn main() {}
