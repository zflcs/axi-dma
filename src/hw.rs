pub const AXIDMA_TX_OFFSET: usize = 0x00;
pub const AXIDMA_RX_OFFSET: usize = 0x30;

use crate::AxiDmaConfig;

pub const AXI_DMA_CONFIG: AxiDmaConfig = AxiDmaConfig {
    device_id: 0,
    base_address: 0x6010_0000,
    has_sts_cntrl_strm: false,
    is_micro_dma: false,
    has_mm2s: true,
    has_mm2s_dre: false,
    mm2s_data_width: 64,
    mm2s_burst_size: 16,
    has_s2mm: true,
    has_s2mm_dre: false,
    s2mm_data_width: 64,
    s2mm_burst_size: 16,
    has_sg: true,
    sg_length_width: 16,
    addr_width: 64,
};