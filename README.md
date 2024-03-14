# axi-dma

The Rust implementation of Xilinx Axi DMA Embedded Driver.

### Supported features

This driver supports the following features:
- [x] Scatter-Gather DMA (SGDMA).
- [ ] Simple DMA.
- [x] Interrupts.
- [ ] Programmable interrupt coalescing for SGDMA.
- [ ] APIs to manage Buffer Descriptors (BD) movement to and from the SGDMA engine.

### Performance

We followed the guide of [the manual of the AxiDMA](https://docs.xilinx.com/r/en-US/pg021_axi_dma) to conduct our test.

We tested the throughput of the tx channel by measuring the first `arvalid` signal on Memory Map side to the `tlast` on the streaming side in the vivado ila.

The completed data sheet is under [throughput](./throughput.xls). The brief result is shown below:

| Data(Byte) | arvalid -> tlast(sampling window) | Throughput(MB/s) |
| ---------- | --------------------------------- | ---------------- |
| 10000      | 7472                              | 401              |

We believe that the rx channel can reach the throughput of [the manual of the AxiDMA](https://docs.xilinx.com/r/en-US/pg021_axi_dma).


### Acknowledgments

Thanks to [Xilinx embeddedsw repository](https://github.com/Xilinx/embeddedsw) and [Gallium70](https://github.com/Gallium70).