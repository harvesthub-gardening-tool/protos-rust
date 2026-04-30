//! Harvest Hub Protocol Buffer definitions

pub mod auth {
    pub mod v1 {
        include!("../auth/v1/auth.v1.rs");
    }
}

pub mod auth {
    pub mod v2 {
        include!("../auth/v2/auth.v2.rs");
    }
}

pub mod garden {
    pub mod v1 {
        include!("../garden/v1/garden.v1.rs");
    }
}

