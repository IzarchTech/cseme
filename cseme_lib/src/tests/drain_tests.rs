use crate::{
    elements::drain::Drain,
    traits::{
        concrete::{VolumeOfBlinding, VolumeOfConcrete},
        excavation::Excavation,
        formwork::AreaOfFormwork,
    },
};
use approx::assert_abs_diff_eq;

fn fixtures() -> [Drain; 2] {
    [
        Drain::new(0.6, 0.6, 1.0, 0.15, Option::from(0.05), Option::from(0.225)),
        Drain::new(1.8, 1.2, 1.0, 0.15, Option::from(0.05), Option::from(0.225)),
    ]
}

#[test]
fn volume_of_concrete_test() {
    let drains = fixtures();
    let expected: [f64; 2] = [0.315, 0.675];

    let mut idx: usize = 0;

    for drain in drains {
        assert_abs_diff_eq!(
            drain.get_volume_of_concrete(),
            expected[idx],
            epsilon = 0.01
        );
        idx += 1;
    }
}

#[test]
fn volume_of_blinding_test() {
    let drains = fixtures();
    let expected: [f64; 2] = [0.068, 0.128];

    let mut idx: usize = 0;

    for drain in drains {
        assert_abs_diff_eq!(
            drain.get_volume_of_blinding(),
            expected[idx],
            epsilon = 0.01
        );
        idx += 1;
    }
}

#[test]
fn area_of_formwork_test() {
    let drains = fixtures();
    let expected: [f64; 2] = [2.7, 5.1];

    let mut idx: usize = 0;

    for drain in drains {
        assert_abs_diff_eq!(drain.get_area_of_formwork(), expected[idx], epsilon = 0.01);
        idx += 1;
    }
}

#[test]
fn volume_of_cart_away_test() {
    let drains = fixtures();
    let expected: [f64; 2] = [0.72, 3.0];

    let mut idx: usize = 0;

    for drain in drains {
        assert_abs_diff_eq!(
            drain.get_volume_of_cart_away(),
            expected[idx],
            epsilon = 0.1
        );
        idx += 1;
    }
}

#[test]
fn volume_of_excavation_test() {
    let drains = fixtures();
    let expected: [f64; 2] = [1.08, 3.57];

    let mut idx: usize = 0;

    for drain in drains {
        assert_abs_diff_eq!(
            drain.get_volume_of_excavation(),
            expected[idx],
            epsilon = 0.1
        );
        idx += 1;
    }
}
