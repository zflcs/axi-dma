# axi-dma

The Rust implementation of Xilinx Axi DMA Embedded Driver.

### Supported features

This driver supports the following features:
- [x] Scatter-Gather DMA (SGDMA).
- [ ] Simple DMA.
- [x] Poll mode.
- [x] Interrupts.
- [x] Programmable interrupt coalescing for SGDMA.
- [ ] APIs to manage Buffer Descriptors (BD) movement to and from the SGDMA engine.
- [x] Combined with the Rust ownership mechanism.
- [x] Async, combined with Rust Future.

### The Scatter-Gather DMA

We support at most 1 transmit channel and 1 receive channel. The related buffer descriptor(BD) management is also simplified.
We follow [the manual of the AxiDMA](https://docs.xilinx.com/r/en-US/pg021_axi_dma) and organize the BDs as a ring, we called BD ring.
The BD ring has fixed capacity once being created and the inner BD management has been integrated into `submit()` and `from_hw()` function.

You can start a transaction by using the `submit()` function.

### Poll mode

Once you submit a buffer, then you will get a related `transfer`, the you can use the `wait()` function to poll the status of the channel. Once the transaction has completed, it will retrieve the completed BD from the channel for next transaction.

We also combined it with the Rust ownership mechanism though the `transfer`.

### Interrupt mode

We also support the interrupt mode. As the same as Poll mode, you will get the `transfer`. Then you should store it in a space(e.g. queue) until the interrupt happend instead of using `wait()` function. In the interrupt handler, you must use the `intr_handler()` of related channel to clear the interrupt otherwise you will step into a tight interrupt loop. After you use the `intr_handler()`, you must free the related `transfer` before return the normal function, otherwise you will get memory leak surprise.

### Async, Interrupt mode combined with Rust future

You must enable the `async` feature. As the same, after you get a `transfer`, you can use the `await` key word. It will try to ask the channel whether the related transaction has been completed. 

If not it will register a waker into the channel and return `pending`. Then you can do other things. After the transaction completed, a interrupt will occur and the related waker will be waken up. Then your can fetch the pending future and continue. 

If the transaction has been completed, it will take the buffer from the `transfer`.

### Usage

You can use the AxiDMA as below: 

```Rust
/***********  initialized  *******************/ 

let _ = AXI_DMA.reset();
// enable cyclic mode
AXI_DMA.cyclic_enable();

// init cyclic block descriptor
let _ = AXI_DMA.tx_channel_create(AXI_NET_CONFIG.tx_bd_cnt);
let _ = AXI_DMA.rx_channel_create(AXI_NET_CONFIG.rx_bd_cnt);

// enable tx & rx intr
AXI_DMA.intr_enable();

/***********  submit a buffer and wait it synchronously *******************/ 
let mut buffer = vec![1u8; MTU].into_boxed_slice();
let len = buffer.len();
let buf_ptr = Box::into_raw(buffer) as *mut _;
let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);

let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();

```

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