/// The Error number
#[derive(Debug)]
pub enum AxiDMAErr {
    /// DMA internal error
    DMAErr,
    /// DMA not initialized
    NotInit,
    /// Buffer descriptor not created or not enough
    BDRingNoList,
    /// The parameter is invalid
    InValidParam,
}