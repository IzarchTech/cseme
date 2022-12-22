use approx::assert_abs_diff_eq;
use crate::{
    elements::drain::Drain,
    traits::concrete::VolumeOfConcrete
};

#[test]
fn volume_of_concrete_test()
{
    let drain = Drain::new(
        0.6,
        0.6,
        1.0,
        0.15,
        Option::from(0.05),
        Option::from(0.225));

    assert_abs_diff_eq!(drain.get_volume_of_concrete(), 0.315)
}

#[test]
#[should_panic(expected = "Invalid input")]
fn should_panic_when_invalid_arguments_are_passed()
{
    Drain::new(
        0.0,
        0.6,
        1.0,
        0.15,
        Option::from(0.05),
        Option::from(0.225));
}