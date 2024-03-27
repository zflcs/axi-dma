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
#[macro_use]
extern crate log;

extern crate alloc;

mod bd;
mod buffer;
mod channel;
mod errno;
mod hw;
mod transfer;

use alloc::sync::Arc;
pub use buffer::BufPtr;
use channel::AxiDMAChannel;
use core::sync::atomic::{AtomicBool, Ordering};
use errno::AxiDMAErr;
use hw::AXI_DMA_CONFIG;
pub use transfer::Transfer;

pub type AxiDMAResult = Result<(), AxiDMAErr>;

/// The AxiDma driver instance structure. An instance must be allocated for each DMA
/// engine in use.
pub struct AxiDma {
    // Immutable
    /// The base address of the AxiDMA
    #[allow(unused)]
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
    pub tx_channel: Option<Arc<AxiDMAChannel>>,
    /// The rx channel
    pub rx_channel: Option<Arc<AxiDMAChannel>>,
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
            Some(Arc::new(AxiDMAChannel::new(channel::Direaction::TX, &cfg)))
        } else {
            None
        };
        let rx_channel = if cfg.has_s2mm {
            Some(Arc::new(AxiDMAChannel::new(channel::Direaction::RX, &cfg)))
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
        }
    }

    /// Get the registers of the AxiDMA
    #[inline]
    #[allow(unused)]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    /// Reset the AxiDMA
    pub fn reset(self: &Arc<Self>) -> AxiDMAResult {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.reset()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.reset()?;
        }
        let mut timeout = AxiDma::RESET_TIMEOUT;
        while timeout > 0 && !self.reset_is_done() {
            timeout -= 1;
        }
        if timeout > 0 {
            self.is_initialized.store(true, Ordering::Relaxed);
        } else {
            error!("AXIDMA: failed reset in intialization");
            return Err(AxiDMAErr::DMAErr);
        }
        Ok(())
    }

    // Check reset is done when both went normal
    fn reset_is_done(self: &Arc<Self>) -> bool {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            if !tx_channel.reset_is_done() {
                return false;
            }
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            if !rx_channel.reset_is_done() {
                return false;
            }
        }
        true
    }

    /// Enable the cyclic mode
    pub fn cyclic_enable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.cyclic_enable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.cyclic_enable();
        }
    }

    /// Disable the cyclic mode
    pub fn cyclic_disable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.cyclic_disable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.cyclic_disable();
        }
    }

    /// Start the AxiDMA
    pub fn start(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Start: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.start()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.start()?;
        }
        Ok(())
    }

    /// Pause the AxiDMA
    pub fn pause(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Pause: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.stop()?;
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.stop()?;
        }
        Ok(())
    }

    /// Resume the AxiDMA
    pub fn resume(self: &Arc<Self>) -> AxiDMAResult {
        if !self.is_initialized.load(Ordering::Relaxed) {
            error!("Resume: Driver not initialized");
            return Err(AxiDMAErr::NotInit);
        }
        self.start()?;
        Ok(())
    }

    /// Disable the interrupt
    pub fn intr_disable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let _ = tx_channel.intr_disable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let _ = rx_channel.intr_disable();
        }
    }

    /// Enable the interrupt
    pub fn intr_enable(self: &Arc<Self>) {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let _ = tx_channel.intr_enable();
        }
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let _ = rx_channel.intr_enable();
        }
    }

    /// Initialize the tx channel
    pub fn tx_channel_create(self: &Arc<Self>, bd_count: usize) -> AxiDMAResult {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            tx_channel.intr_disable();
            tx_channel.create(bd_count)?;
            return Ok(());
        }
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Initialize the rx channel
    pub fn rx_channel_create(self: &Arc<Self>, bd_count: usize) -> AxiDMAResult {
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            rx_channel.intr_disable();
            rx_channel.create(bd_count)?;
            return Ok(());
        }
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Submit a buffer to the tx channel
    pub fn tx_submit(self: &Arc<Self>, buffer: BufPtr) -> Result<Transfer, AxiDMAErr> {
        if let Some(tx_channel) = self.tx_channel.as_ref() {
            let transfer = Transfer::new(tx_channel.submit(buffer)?, tx_channel.clone());
            tx_channel.to_hw()?;
            return Ok(transfer);
        }
        error!("axidma::tx_submit: no tx ring!");
        Err(AxiDMAErr::BDRingNoList)
    }

    /// Submit a buffer to the rx channel
    pub fn rx_submit(self: &Arc<Self>, buffer: BufPtr) -> Result<Transfer, AxiDMAErr> {
        if let Some(rx_channel) = self.rx_channel.as_ref() {
            let transfer = Transfer::new(rx_channel.submit(buffer)?, rx_channel.clone());
            rx_channel.to_hw()?;
            return Ok(transfer);
        }
        error!("axidma::rx_submit: no rx ring!");
        Err(AxiDMAErr::BDRingNoList)
    }
}

///
#[inline]
pub fn io_fence() {
    unsafe {
        core::arch::asm!("fence iorw,iorw");
    }
}
