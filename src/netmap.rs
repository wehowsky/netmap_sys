use libc::{c_int, c_uint, c_ulong, c_char, timeval, ssize_t, IF_NAMESIZE};

pub const IFNAMSIZ: usize = 16;
pub const NETMAP_REQ_IFNAMSIZ: usize = 64;

pub const NETMAP_API: c_int = 11;
pub const NETMAP_MIN_API: c_int = 11;
pub const NETMAP_MAX_API: c_int = 15;

pub const NM_CACHE_ALIGN: c_int = 128;


/* Header common to all request options. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_option {
    /* Pointer to the next option. */
    pub nro_next: u64,
    /* Option type. */
    pub nro_reqtype: u32,
    /* (out) status of the option:
     * 0: recognized and processed
     * !=0: errno value
     */
    pub nro_status: u32,
    /* Option size, used only for options that can have variable size
     * (e.g. because they contain arrays). For fixed-size options this
     * field should be set to zero. */
    pub nro_size: u64,
}


/* Header common to all requests. Do not reorder these fields, as we need
 * the second one (nr_reqtype) to know how much to copy from/to userspace. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_header {
    pub nr_version: u16,	        /* API version */
    pub nr_reqtype: u16,        	/* nmreq type (NETMAP_REQ_*) */
    pub nr_reserved: u32,       	/* must be zero */
    pub nr_name: [c_char; NETMAP_REQ_IFNAMSIZ],/* port name */
    pub nr_options: u64,        	/* command-specific options */
    pub nr_body: u64,   	        /* ptr to nmreq_xyz struct */
}


/*
 * nr_reqtype: NETMAP_REQ_REGISTER
 * Bind (register) a netmap port to this control device.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_register {
    pub nr_offset: u64,
    /* nifp offset in the shared region */
    pub nr_memsize: u64,
    /* size of the shared region */
    pub nr_tx_slots: u32,
    /* slots in tx rings */
    pub nr_rx_slots: u32,
    /* slots in rx rings */
    pub nr_tx_rings: u16,
    /* number of tx rings */
    pub nr_rx_rings: u16,
    /* number of rx rings */
    pub nr_host_tx_rings: u16,
    /* number of host tx rings */
    pub nr_host_rx_rings: u16,
    /* number of host rx rings */

    pub nr_mem_id: u16,
    /* id of the memory allocator */
    pub nr_ringid: u16,
    /* ring(s) we care about */
    pub nr_mode: u32,
    /* specify NR_REG_* modes */
    pub nr_extra_bufs: u32,
    /* number of requested extra buffers */

    pub nr_flags: u64,
    /* additional flags (see below) */
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct netmap_slot {
    pub buf_idx: u32,
    pub len: u16,
    pub flags: u16,
    pub ptr: u64,
}

pub const NS_BUF_CHANGED: u16 = 0x0001;
pub const NS_REPORT: u16 = 0x0002;
pub const NS_FORWARD: u16 = 0x0004;
pub const NS_NO_LEARN: u16 = 0x0008;
pub const NS_INDIRECT: u16 = 0x0010;
pub const NS_MOREFRAG: u16 = 0x0020;

pub const NS_PORT_SHIFT: c_int = 8;
pub const NS_PORT_MASK: c_int = (0xff << NS_PORT_SHIFT);

// FIXME NS_RFRAGS

#[repr(C)]
#[derive(Copy)]
pub struct netmap_ring {
    pub buf_ofs: i64,
    pub num_slots: u32,
    pub nr_buf_size: u32,
    pub ringid: u16,
    pub dir: u16,

    pub head: u32,
    pub cur: u32,
    pub tail: u32,

    pub flags: u32,

    pub ts: timeval,

    pub offset_mask: u64,
    pub buf_align: u64,
    pub sem: [u8; 128], // FIXME  __attribute__((__aligned__(NM_CACHE_ALIGN)))

    pub slot: [netmap_slot; 0], // FIXME Check struct size/field alignment
}

impl Clone for netmap_ring {
    fn clone(&self) -> netmap_ring {
        *self
    }
}

pub const NR_TIMESTAMP: u32 = 0x0002;
pub const NR_FORWARD: u32 = 0x0004;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct netmap_if {
    pub ni_name: [c_char; IFNAMSIZ],
    pub ni_version: u32,
    pub ni_flags: u32,

    pub ni_tx_rings: u32,
    pub ni_rx_rings: u32,

    pub ni_bufs_head: u32,
    pub	ni_host_tx_rings: u32, /* number of SW tx rings */
    pub	ni_host_rx_rings: u32, /* number of SW tx rings */
    pub ni_spare1: [u32; 3],

    pub ring_ofs: [ssize_t; 0], // FIXME Check this is right, see above
}

pub const NI_PRIV_MEM: c_int = 0x1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nmreq {
    pub nr_name: [c_char; IFNAMSIZ],
    pub nr_version: u32,
    pub nr_offset: u32,
    pub nr_memsize: u32,
    pub nr_tx_slots: u32,
    pub nr_rx_slots: u32,
    pub nr_tx_rings: u16,
    pub nr_rx_rings: u16,

    pub nr_ringid: u16,

    pub nr_cmd: u16,
    pub nr_arg1: u16,
    pub nr_arg2: u16,
    pub nr_arg3: u32,
    pub nr_flags: u32,

    pub spare2: [u32; 1],
}

/*
 * nr_reqtype: NETMAP_REQ_POOLS_INFO_GET
 * Get info about the pools of the memory allocator of the netmap
 * port specified by hdr.nr_name and nr_mem_id. The netmap control
 * device used for this operation does not need to be bound to a netmap
 * port.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_pools_info {
    pub nr_memsize: u64,
    pub nr_mem_id: u16, /* in/out argument */
    pub pad1: [u16; 3],
    pub nr_if_pool_offset: u64,
    pub nr_if_pool_objtotal: u32,
    pub nr_if_pool_objsize: u32,
    pub nr_ring_pool_offset: u64,
    pub nr_ring_pool_objtotal: u32,
    pub nr_ring_pool_objsize: u32,
    pub nr_buf_pool_offset: u64,
    pub nr_buf_pool_objtotal: u32,
    pub nr_buf_pool_objsize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_opt_sync_kloop_mode {
    pub nro_opt: nmreq_option,	/* common header */
    pub mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmreq_opt_extmem {
    pub nro_opt: nmreq_option,	/* common header */
    pub nro_usrptr: u64,	/* (in) ptr to usr memory */
    pub nro_info: nmreq_pools_info,	/* (in/out) */
}


pub const NETMAP_HW_RING: c_int = 0x4000;
pub const NETMAP_SW_RING: c_int = 0x2000;

pub const NETMAP_RING_MASK: c_int = 0x0fff;

pub const NETMAP_NO_TX_POLL: c_int = 0x1000;

pub const NETMAP_DO_RX_POLL: c_int = 0x8000;

pub const NETMAP_BDG_ATTACH: c_int = 1;
pub const NETMAP_BDG_DETACH: c_int = 2;
pub const NETMAP_BDG_REGOPS: c_int = 3;
pub const NETMAP_BDG_LIST: c_int = 4;
pub const NETMAP_BDG_VNET_HDR: c_int = 5;
pub const NETMAP_BDG_OFFSET: c_int = NETMAP_BDG_VNET_HDR;
pub const NETMAP_BDG_NEWIF: c_int = 6;
pub const NETMAP_BDG_DELIF: c_int = 7;

pub const NETMAP_BDG_HOST: c_int = 1;

pub const NR_REG_MASK: c_int = 0xf;

pub const NR_REG_DEFAULT: u32 = 0;
pub const NR_REG_ALL_NIC: u32 = 1;
pub const NR_REG_SW: u32 = 2;
pub const NR_REG_NIC_SW: u32 = 3;
pub const NR_REG_ONE_NIC: u32 = 4;
pub const NR_REG_PIPE_MASTER: u32 = 5;
pub const NR_REG_PIPE_SLAVE: u32 = 6;

pub const NR_MONITOR_TX: u32 = 0x100;
pub const NR_MONITOR_RX: u32 = 0x200;
pub const NR_ZCOPY_MON: u32 = 0x400;
pub const NR_EXCLUSIVE: u32 = 0x800;
pub const NR_PTNETMAP_HOST: u32 = 0x1000;
pub const NR_RX_RINGS_ONLY: u32 = 0x2000;
pub const NR_TX_RINGS_ONLY: u32 = 0x4000;
pub const NR_ACCEPT_VNET_HDR: u32 = 0x8000;
pub const  NR_DO_RX_POLL: u32 = 0x10000;
pub const  NR_NO_TX_POLL: u32 = 0x20000;

#[cfg(target_os = "linux")]
pub const NIOCGINFO: c_ulong = 3225184657;
#[cfg(target_os = "linux")]
pub const NIOCREGIF: c_ulong = 3225184658;
#[cfg(target_os = "linux")]
pub const NIOCTXSYNC: c_uint = 27028;
#[cfg(target_os = "linux")]
pub const NIOCRXSYNC: c_uint = 27029;
#[cfg(target_os = "linux")]
pub const NIOCCONFIG: c_ulong = 3239078294;

#[cfg(target_os = "freebsd")]
pub const NIOCGINFO: c_ulong = 3225184657;
#[cfg(target_os = "freebsd")]
pub const NIOCREGIF: c_ulong = 3225184658;
#[cfg(target_os = "freebsd")]
pub const NIOCTXSYNC: c_uint = 536897940;
#[cfg(target_os = "freebsd")]
pub const NIOCRXSYNC: c_uint = 536897941;
#[cfg(target_os = "freebsd")]
pub const NIOCCONFIG: c_ulong = 3239078294;

#[inline(always)]
pub unsafe fn nm_ring_empty(ring: *mut netmap_ring) -> bool {
    (*ring).head == (*ring).tail
}

pub const NM_IFRDATA_LEN: usize = 256;

#[repr(C)]
#[derive(Copy)]
pub struct nm_ifreq {
    pub nifr_name: [c_char; IFNAMSIZ],
    pub data: [c_char; NM_IFRDATA_LEN],
}

impl Clone for nm_ifreq {
    fn clone(&self) -> nm_ifreq {
        *self
    }
}

