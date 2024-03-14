//! This is the driver API for the AXI DMA engine.
//! 
//! For a full description of DMA features, please see the hardware spec. 
//! This driver supports the following features:
//!     - [x] Scatter-Gather DMA (SGDMA)
//!     - [ ] Simple DMA
//!     - [x] Interrupts
//!     - [ ] Programmable interrupt coalescing for SGDMA
//!     - [ ] APIs to manage Buffer Descriptors (BD) movement to and from the SGDMA engine
//! 
//! ### Simple DMA
//! 
//! Simple DMA allows the application to define a single transaction between DMA
//! and Device. It has two channels: one from the DMA to Device and the other
//! from Device to DMA. Application has to set the buffer address and
//! length fields to initiate the transfer in respective channel.
//! 
//! ### Transactions
//! 
//! The object used to describe a transaction is referred to as a Buffer
//! Descriptor (BD). Buffer descriptors are allocated in the user application.
//! The user application needs to set buffer address, transfer length, and
//! control information for this transfer. The control information includes
//! SOF and EOF. Definition of those masks are in [xaxidma.h](https://github.com/Xilinx/embeddedsw/blob/master/XilinxProcessorIPLib/drivers/axidma/src/xaxidma.h) 
//! 
//! ### Scatter-Gather DMA
//! 
//! SGDMA allows the application to define a list of transactions in memory which
//! the hardware will process without further application intervention. During
//! this time, the application is free to continue adding more work to keep the
//! Hardware busy.
//! 
//! User can check for the completion of transactions through polling the
//! hardware, or interrupts.
//! 
//! SGDMA processes whole packets. A packet is defined as a series of
//! data bytes that represent a message. SGDMA allows a packet of data to be
//! broken up into one or more transactions. For example, take an Ethernet IP
//! packet which consists of a 14 byte header followed by a 1 or more bytes of
//! payload. With SGDMA, the application may point a BD to the header and another
//! BD to the payload, then transfer them as a single message. This strategy can
//! make a TCP/IP stack more efficient by allowing it to keep packet header and
//! data in different memory regions instead of assembling packets into
//! contiguous blocks of memory.
//!
//! <b>BD Ring Management</b>
//!
//! BD rings are shared by the software and the hardware.
//!
//! The hardware expects BDs to be setup as a linked list. The DMA hardware walks
//! through the list by following the next pointer field of a completed BD.
//! The hardware stops processing when the just completed BD is the same as the
//! BD specified in the Tail Ptr register in the hardware.
//!
//! The last BD in the ring is linked to the first BD in the ring.
//!
//! All BD management are done inside the driver. The user application should not
//! directly modify the BD fields. Modifications to the BD fields should always
//! go through the specific API functions.
//!
//! Within the ring, the driver maintains four groups of BDs. Each group consists
//! of 0 or more adjacent BDs:
//!
//!   - Free: The BDs that can be allocated by the application with
//!     XAxiDma_BdRingAlloc().
//!
//!   - Pre-process: The BDs that have been allocated with
//!     XAxiDma_BdRingAlloc(). These BDs are under application control. The
//!     application modifies these BDs through driver API to prepare them
//!     for DMA transactions.
//!
//!   - Hardware: The BDs that have been enqueued to hardware with
//!     XAxiDma_BdRingToHw(). These BDs are under hardware control and may be in a
//!     state of awaiting hardware processing, in process, or processed by
//!     hardware. It is considered an error for the application to change BDs
//!     while they are in this group. Doing so can cause data corruption and lead
//!     to system instability.
//!
//!   - Post-process: The BDs that have been processed by hardware and have
//!     been extracted from the Hardware group with XAxiDma_BdRingFromHw().
//!     These BDs are under application control. The application can check the
//!     transfer status of these BDs. The application use XAxiDma_BdRingFree()
//!     to put them into the Free group.
//!
//! BDs are expected to transition in the following way for continuous
//! DMA transfers:
//! <pre>
//!
//!         XAxiDma_BdRingAlloc()                   XAxiDma_BdRingToHw()
//!   Free ------------------------> Pre-process ----------------------> Hardware
//!                                                                      |
//!    /|\                                                               |
//!     |   XAxiDma_BdRingFree()                  XAxiDma_BdRingFromHw() |
//!     +--------------------------- Post-process <----------------------+
//!
//! </pre>
//!
//! <b>SGDMA Descriptor Ring Creation</b>
//!
//! BD ring is created using *x_channel_create(). The memory for the BD ring
//! is allocated by the application, and it has to be contiguous. Physical
//! address is required to setup the BD ring.
//!
//! <b>Descriptor Ring State Machine</b>
//!
//! There are two states of the BD ring:
//!
//!   - HALTED (H), where hardware is not running
//!
//!   - NOT HALTED (NH), where hardware is running
//!
//! The following diagram shows the state transition for the DMA engine:
//!
//! <pre>
//!   _____                                                      ______
//!   |   |  submit(), or start(), or resume()                   |    |
//!   | H |----------------------------------------------------->| NH |
//!   |   |<-----------------------------------------------------|    |
//!   -----   pause() or reset()                                 ------
//! </pre>
//!
//! <b>Interrupt Coalescing</b>
//!
//! SGDMA provides control over the frequency of interrupts through interrupt
//! coalescing. The DMA engine provides two ways to tune the interrupt
//! coalescing:
//!
//! - The packet threshold counter. Interrupt will fire once the
//!   programmable number of packets have been processed by the engine.
//!
//! - The packet delay timer counter. Interrupt will fire once the
//!   programmable amount of time has passed after processing the last packet,
//!   and no new packets to process. Note that the interrupt will only fire if
//!   at least one packet has been processed.
//!
//! <b> Interrupt </b>
//!
//! Interrupts are handled by the user application. Each DMA channel has its own
//! interrupt ID. The driver provides APIs to enable/disable interrupt,
//! and tune the interrupt frequency regarding to packet processing frequency.
//!
//! <b> Software Initialization </b>
//!
//! To use the SG mode DMA engine for transfers, the following setup are
//! required:
//!
//! - DMA Initialization using new(cfg) function. This step
//!   initializes a driver instance for the given DMA engine and resets the
//!   engine.
//!
//! - BD Ring creation. A BD ring is needed per DMA channel and can be built by
//!   calling create().
//!
//! - Enable interrupts if chose to use interrupt mode. The application is
//!   responsible for setting up the interrupt system, which includes providing
//!   and connecting interrupt handlers and call back functions, before
//!   enabling the interrupts.
//!
//! - Start a DMA transfer: Call submit() to start a transfer. 
//!   Calling submit() when a DMA channel is not running 
//!   will start the DMA channel.
//!
//! <b> How to start DMA transactions </b>
//!
//! The user application uses submit() to submit BDs to the hardware
//! to start DMA transfers.
//!
//! <b> Software Post-Processing on completed DMA transactions </b>
//!
//! If the interrupt system has been set up and the interrupts are enabled,
//! a DMA channels notifies the software about the completion of a transfer
//! through interrupts. Otherwise, the user application can poll for
//! completions of the BDs, using from_hw().
//!
//! - Once BDs are finished by a channel, the application first needs to fetch
//!   them from the channel using from_hw().
//!
//! - On the TX side, the application now could free the data buffers attached to
//!   those BDs as the data in the buffers has been transmitted.
//!
//! - On the RX side, the application now could use the received data in the
//!	buffers attached to those BDs.
//!
//! - For both channels, completed BDs need to be put back to the Free group,
//!   so they can be used for future transactions.
//!
//! - On the RX side, it is the application's responsibility to have BDs ready
//!   to receive data at any time. Otherwise, the RX channel refuses to
//!   accept any data if it has no RX BDs.
//!
//!
//! <b> Cache Coherency </b>
//!
//! This driver expects all application buffers attached to BDs to be in cache
//! coherent memory. If cache is used in the system, buffers for transmit MUST
//! be flushed from the cache before passing the associated BD to this driver.
//! Buffers for receive MUST be invalidated before accessing the data.
//!
//! <b> Alignment </b>
//!
//! For BDs:
//!
//! Minimum alignment is defined by the constant [XAXIDMA_BD_MINIMUM_ALIGNMENT(0x40)](https://github.com/Xilinx/embeddedsw/blob/master/XilinxProcessorIPLib/drivers/axidma/src/xaxidma_hw.h#L58).
//! This is the smallest alignment allowed by both hardware and software for them
//! to properly work.
//!
//! If the descriptor ring is to be placed in cached memory, alignment also MUST
//! be at least the processor's cache-line size. Otherwise, system instability
//! occurs. For alignment larger than the cache line size, multiple cache line
//! size alignment is required.
//!
//! Aside from the initial creation of the descriptor ring (see
//! create_channel()), there are no other run-time checks for proper
//! alignment of BDs.
//!
//! For application data buffers:
//!
//! Application data buffers may reside on any alignment if DRE is built into the
//! hardware. Otherwise, application data buffer must be word-aligned. The word
//! is defined by XPAR_AXIDMA_0_M_AXIS_MM2S_TDATA_WIDTH for transmit and
//! XPAR_AXIDMA_0_S_AXIS_S2MM_TDATA_WIDTH for receive.
//!
//! For scatter gather transfers that have more than one BDs in the chain of BDs,
//! Each BD transfer length must be multiple of word too. Otherwise, internal
//! error happens in the hardware.
//!
//! <b> Error Handling </b>
//!
//! The DMA engine will halt on all error conditions. It requires the software
//! to do a reset before it can start process new transfer requests.
//!
//! <b> Restart After Stopping </b>
//!
//! After the DMA engine has been stopped (through reset or reset after an error)
//! the software keeps track of the current BD pointer when reset happens, and
//! processing of BDs can be resumed through start().
//!
//! <b> Limitations </b>
//!
//! This driver does not have any mechanisms for mutual exclusion. It is up to
//! the application to provide this protection.
//!
//! <b> Hardware Defaults & Exclusive Use </b>
//!
//! After the initialization or reset, the DMA engine is in the following
//! default mode:
//! - All interrupts are disabled.
//!
//! - Interrupt coalescing counter is 1.
//!
//! - The DMA engine is not running (halted). Each DMA channel is started
//!   separately, using start() if no BDs are setup for transfer
//!   yet, or submit() otherwise.
//!
//! The driver has exclusive use of the registers and BDs. All accesses to the
//! registers and BDs should go through the driver interface.
//!
//! <b> Debug Print </b>
//!
//! To see the debug print for the driver, please put "TRACE" as the extra
//! compiler flags in software platform settings.
//! 

#![no_std]
#[deny(missing_docs)]
#[deny(unused)]
extern crate alloc;

mod bd;
mod channel;
mod errno;
mod transfer;
mod hw;

use core::{ops::Deref, pin::Pin, sync::atomic::AtomicBool};
#[cfg(feature = "async")]
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use channel::AxiDMAChannel;
use errno::AxiDMAErr;
use hw::AXI_DMA_CONFIG;
use spin::Mutex;
use transfer::{RxTransfer, TxTransfer};
use core::sync::atomic::Ordering;

type AxiDMAResult = Result<(), AxiDMAErr>;

/// The AxiDma driver instance structure. An instance must be allocated for each DMA
/// engine in use.
pub struct AxiDma {
    /// The base address of the AxiDMA
    base_address: usize,
    /// Has Scatter Gather mode
    #[allow(unused)]
    has_sg: bool,
    /// Whether the micro dma mode is enable
    #[allow(unused)]
    is_micro_dma: bool,
    /// the width of address
    #[allow(unused)]
    addr_width: isize,

    // Mutable
    /// Whether the AxiDMA is initialized
    is_initialized: AtomicBool,
    /// The tx channel
    tx_channel: Option<Mutex<AxiDMAChannel>>,
    /// The rx channel
    rx_channel: Option<Mutex<AxiDMAChannel>>,

    #[cfg(feature = "async")]
    /// The queue of wakers of the tx channel
    pub tx_wakers: Mutex<VecDeque<Waker>>,
    #[cfg(feature = "async")]
    /// The queue of wakers of the rx channel
    pub rx_wakers: Mutex<VecDeque<Waker>>,
}

/// The configuration structure for AXI DMA engine.
#[derive(Debug)]
pub struct AxiDmaConfig {
    /// The base address of the AxiDMA
    pub base_address: usize,
    /// The address offset of rx channel in the AxiDMA
    pub rx_channel_offset: usize,
    /// The address offset of tx channel in the AxiDMA
    pub tx_channel_offset: usize,
    /// Whether the status/control stream
    pub has_sts_cntrl_strm: bool,
    /// Whether the micro dma mode is enable
    pub is_micro_dma: bool,
    /// Has tx channel
    pub has_mm2s: bool,
    /// Whether the tx channel has enabled the data realignment 
    pub has_mm2s_dre: bool,
    /// The data width of tx channel
    pub mm2s_data_width: usize,
    /// The burst size of tx channel
    pub mm2s_burst_size: usize,
    /// Has rx channel
    pub has_s2mm: bool,
    /// Whether the rx channel has enabled the data realignment 
    pub has_s2mm_dre: bool,
    /// The data width of rx channel
    pub s2mm_data_width: usize,
    /// The burst size of tx channel
    pub s2mm_burst_size: usize,
    /// Has Scatter Gather mode
    pub has_sg: bool,
    /// The width of the buffer length field 
    pub sg_length_width: usize,
    /// the width of address
    pub addr_width: isize,
}

impl Default for AxiDma {
    fn default() -> Self {
        Self::new(AXI_DMA_CONFIG)
    }
}

impl AxiDma {
    /// Reset time out
    const RESET_TIMEOUT: isize = 500;

    /// Create the AxiDMA instance according to the configuration
    pub fn new(cfg: AxiDmaConfig) -> Self {
        let tx_channel = if cfg.has_mm2s {
            Some(Mutex::new(AxiDMAChannel::new(channel::Direaction::TX, &cfg)))
        } else {
            None
        };
        let rx_channel = if cfg.has_s2mm {
            Some(Mutex::new(AxiDMAChannel::new(channel::Direaction::RX, &cfg)))
        } else {
            None
        };
        Self {
            base_address: cfg.base_address,
            has_sg: cfg.has_sg,
            is_micro_dma: cfg.is_micro_dma,
            addr_width: cfg.addr_width,
            tx_channel,
            rx_channel,
            is_initialized: AtomicBool::new(false),
            #[cfg(feature = "async")]
            tx_wakers: Mutex::new(VecDeque::new()),
            #[cfg(feature = "async")]
            rx_wakers: Mutex::new(VecDeque::new()),
        }
    }

    /// Get the registers of the AxiDMA
    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    /// Reset the AxiDMA
    pub fn reset(self: &Arc<Self>) -> AxiDMAResult {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().reset()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().reset()?;
        }
        let mut timeout = AxiDma::RESET_TIMEOUT;
        while timeout > 0 && !self.reset_is_done() {
            timeout -= 1;
        }
        if timeout > 0 {
            self.is_initialized.store(true, Ordering::Relaxed);
        } else {
            log::error!("AXIDMA: failed reset in intialization");
            return Err(AxiDMAErr::DMAErr);
        }
        Ok(())
    }

    // Check reset is done when both went normal
    fn reset_is_done(self: &Arc<Self>) -> bool {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            if !tx_channel.lock().reset_is_done() {
                return false;
            }
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            if !rx_channel.lock().reset_is_done() {
                return false;
            }
        }
        true
    }

    /// Enable the cyclic mode
    pub fn cyclic_enable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().cyclic_enable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().cyclic_enable();
        }
    }

    /// Disable the cyclic mode
    pub fn cyclic_disable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().cyclic_disable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().cyclic_disable();
        }
    }

    /// Start the AxiDMA
    pub fn start(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            log::error!("Start: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().start()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().start()?;
        }
        Ok(())
    }

    /// Pause the AxiDMA
    pub fn pause(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            log::error!("Pause: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().stop()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().stop()?;
        }
        Ok(())
    }

    /// Resume the AxiDMA
    pub fn resume(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            log::error!("Resume: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        self.start()?;
        Ok(())
    }

    /// Disable the interrupt
    pub fn intr_disable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let _ = tx_channel.lock().intr_disable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let _ = rx_channel.lock().intr_disable();
        }
    }

    /// Enable the interrupt
    pub fn intr_enable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let _ = tx_channel.lock().intr_enable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let _ = rx_channel.lock().intr_enable();
        }
    }

    /// Initialize the tx channel
    pub fn tx_channel_create(self: &Arc<Self>, bd_count: usize) -> AxiDMAResult {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let mut tx_channel = tx_channel.lock();
            tx_channel.intr_disable();
            tx_channel.create(bd_count)?;
        }
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Initialize the rx channel
    pub fn rx_channel_create(self: &Arc<Self>, bd_count: usize) -> AxiDMAResult {
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let mut rx_channel = rx_channel.lock();
            rx_channel.intr_disable();
            rx_channel.create(bd_count)?;
        }
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Submit a buffer to the tx channel
    pub fn tx_submit<B>(self: &Arc<Self>, buffer: Pin<B>) -> Result<TxTransfer<B>, AxiDMAErr>
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().submit(&buffer)?;
            return Ok(TxTransfer::new(buffer, self.clone()));
        }
        log::error!("axidma::tx_from_hw: no tx ring!");
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Submit a buffer to the rx channel
    pub fn rx_submit<B>(self: &Arc<Self>, buffer: Pin<B>) -> Result<RxTransfer<B>, AxiDMAErr>
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().submit(&buffer)?;
            return Ok(RxTransfer::new(buffer, self.clone()));
        }
        log::error!("axidma::tx_from_hw: no tx ring!");
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Retrieve the completed buffer descriptor from tx channel
    pub fn tx_from_hw(self: &Arc<Self>) -> AxiDMAResult {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.lock().from_hw()
        } else {
            log::error!("axidma::tx_from_hw: no tx ring!");
            Err(AxiDMAErr::BDRingNoList)
        }
        
    }

    /// Retrieve the completed buffer descriptor from rx channel
    pub fn rx_from_hw(&self) -> AxiDMAResult {
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.lock().from_hw()
        } else {
            log::error!("axidma::rx_from_hw: no rx ring!");
            Err(AxiDMAErr::BDRingNoList)
        }
    }

    // Wait the tx channel completing a transaction synchronously.
    pub fn tx_wait(self: &Arc<Self>) {
        let mut status = self.hardware().mm2s_dmasr().read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().mm2s_dmasr().read();
        }
    }

    // Wait the rx channel completing a transaction synchronously.
    pub fn rx_wait(self: &Arc<Self>) {
        let mut status = self.hardware().s2mm_dmasr().read();
        while status.ioc_irq().is_no_intr() && status.dly_irq().is_no_intr() && status.err_irq().is_no_intr() {
            status = self.hardware().s2mm_dmasr().read();
        }
    }
}

/// The interrupt handler of AxiDMA
pub struct AxiDmaIntr {
    /// The base address, it is same as AxiDMA
    base_address: usize,
}

impl AxiDmaIntr {

    /// Create a new AxiDMAIntr
    pub fn new(base_address: usize) -> Arc<Self> {
        Arc::new(Self { base_address })
    }

    /// Get the register of the AxiDMA
    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    /// The interrupt of tx channel
    pub fn tx_intr_handler(self: &Arc<Self>) -> bool {
        let sr = &self.hardware().mm2s_dmasr();
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            log::error!("axidma_intr: tx err intr detected");
            self.tx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
            return false;
        }
        if sr.read().ioc_irq().is_detected() {
            log::trace!("axidma_intr: tx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
        }
        if sr.read().dly_irq().is_detected() {
            log::trace!("axidma_intr: tx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
        }
        true
    }

    /// The interrupt of rx channel
    pub fn rx_intr_handler(self: &Arc<Self>) -> bool {
        let sr = &self.hardware().s2mm_dmasr();
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            log::error!("axidma: rx err intr detected");
            self.rx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
            return false;
        }
        if sr.read().ioc_irq().is_detected() {
            log::trace!("axidma_intr: rx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
        }
        if sr.read().dly_irq().is_detected() {
            log::trace!("axidma_intr: rx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
        }
        true
    }

    /// Dump the register of tx channel
    pub fn tx_dump_regs(self: &Arc<Self>) {
        let hw = self.hardware();
        log::info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.mm2s_dmacr().read().bits(),
            hw.mm2s_dmasr().read().bits()
        );
        log::info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.mm2s_curdesc_msb().read().bits(),
            hw.mm2s_curdesc_msb().read().bits()
        );
        log::info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.mm2s_taildesc_msb().read().bits(),
            hw.mm2s_taildesc().read().bits()
        );
    }

    /// Dump the register of rx channel
    pub fn rx_dump_regs(self: &Arc<Self>) {
        let hw = self.hardware();
        log::info!(
            "CR: 0b{:b}, SR: 0b{:b}",
            hw.s2mm_dmacr().read().bits(),
            hw.s2mm_dmasr().read().bits()
        );
        log::info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.s2mm_curdesc_msb().read().bits(),
            hw.s2mm_curdesc_msb().read().bits()
        );
        log::info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.s2mm_taildesc_msb().read().bits(),
            hw.s2mm_taildesc().read().bits()
        );
    }
}

///
#[inline]
pub fn io_fence() {
    unsafe {
        core::arch::asm!("fence iorw,iorw");
    }
}