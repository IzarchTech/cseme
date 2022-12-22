use crate::traits::{
    concrete::{VolumeOfBlinding, VolumeOfConcrete},
    excavation::Excavation,
};

pub(crate) struct Drain {
    width: f64,
    span: f64,
    depth: f64,
    thickness: f64,
    blinding_thickness: f64,
    working_allowance: f64,
}

impl Drain {
    pub fn new(
        width: f64,
        depth: f64,
        span: f64,
        thickness: f64,
        blinding_thickness: Option<f64>,
        working_allowance: Option<f64>,
    ) -> Self {
        if width <= 0.0f64 || depth < 0.0 || span <= 0.0 || thickness <= 0.0 {
            panic!("Invalid input")
        }
        Drain {
            width,
            span,
            depth,
            thickness,
            blinding_thickness: blinding_thickness.unwrap_or(0.0),
            working_allowance: working_allowance.unwrap_or(0.0),
        }
    }

    fn get_drain_width(&self) -> f64 {
        self.width + (2f64 * self.thickness)
    }

    fn get_drain_depth(&self) -> f64 {
        self.depth + self.thickness
    }

    fn get_excavation_depth(&self) -> f64 {
        self.get_drain_depth() + self.blinding_thickness
    }

    fn get_excavation_width(&self) -> f64 {
        self.get_drain_width() + (2f64 * self.working_allowance)
    }
}

impl VolumeOfConcrete for Drain {
    fn get_volume_of_concrete(&self) -> f64 {
        ((2f64 * self.depth) + self.get_drain_width()) * self.thickness * self.span
    }
}

impl VolumeOfBlinding for Drain {
    fn get_volume_of_blinding(&self) -> f64 {
        self.get_excavation_width() * self.get_excavation_depth() * self.span
    }
}

impl Excavation for Drain {
    fn get_volume_of_excavation(&self) -> f64 {
        self.get_excavation_width() * self.get_excavation_depth() * self.span
    }

    fn get_volume_of_cart_away(&self) -> f64 {
        self.get_excavation_depth() * self.get_drain_width() * self.span
    }
}
