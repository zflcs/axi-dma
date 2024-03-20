use crate::AxiDmaConfig;

/// The default configuration of the AxiDMA
pub const AXI_DMA_CONFIG: AxiDmaConfig = AxiDmaConfig {
    base_address: 0x6010_0000,
    rx_channel_offset: 0x30,
    tx_channel_offset: 0,
    has_sts_cntrl_strm: false,
    is_micro_dma: false,
    has_mm2s: true,
    has_mm2s_dre: false,
    mm2s_data_width: 32,
    mm2s_burst_size: 16,
    has_s2mm: true,
    has_s2mm_dre: false,
    s2mm_data_width: 32,
    s2mm_burst_size: 16,
    has_sg: true,
    sg_length_width: 16,
    addr_width: 32,
};
