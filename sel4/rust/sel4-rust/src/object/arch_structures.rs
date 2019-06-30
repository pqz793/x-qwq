#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
use crate::types;
use crate::types::bool_t;
use crate::types::word_t;
use crate::structures::cap_t;
use crate::structures::cap_tag_t;
use crate::structures::thread_state_t;

//generated/arch/object/structures_gen.h
#[repr(C)]
pub struct pde{
    words:[u64;1]
}
pub type pde_t=pde;

#[repr(C)]
pub struct pte{
    words:[u64;1]
}
pub type pte_t=pte;

const cap_zombie_cap:u64=18;
#[inline]
pub fn cap_zombie_cap_new(capZombieID:u64, capZombieType:u64)->cap_t{
    cap_t{
        words:[0|(cap_zombie_cap&31u64)<<59|(capZombieType&127u64)<<0,
        0|capZombieID<<0]
    }
}

#[inline]
pub fn cap_zombie_cap_get_capZombieType(cap:cap_t)->u64{
    cap.words[0]&127u64
}

#[inline]
pub fn cap_zombie_cap_get_capZombieID(cap:cap_t)->u64{
    cap.words[1]&0xffffffffffffffffu64
}

#[inline]
pub fn cap_zombie_cap_set_capZombieID(mut cap:cap_t,v64:u64)->cap_t{
    cap.words[1] &= ! 0xffffffffffffffffu64;
    cap.words[1] |= v64 & 0xffffffffffffffffu64;
    cap
}

#[inline]
pub fn cap_frame_cap_get_capFSize(cap:cap_t)->u64{
    (cap.words[0]>>59) & 0x1fu64
}

#[inline]
pub fn cap_get_capType(cap:cap_t)->u64{
    (cap.words[0]>>59) & 0x1fu64
}

#[inline]
pub fn cap_endpoint_cap_get_capEPPtr(cap:cap_t)->u64{
    let mut ret:u64=cap.words[0] & 0xffffffffffffu64;
    if (ret & (1u64 << 47))!=0 {
        ret |= 0xffff000000000000;
    }
    ret
}

#[inline]
pub fn isArchCap(cap:cap_t)->word_t{
    cap_get_capType(cap) % 2
}

#[inline]
pub fn cap_cnode_cap_get_capCNodeRadix(cap:cap_t)->u64{
    (cap.words[0] & 0x1f800000000000u64)>>47
}

#[inline]
pub fn cap_endpoint_cap_get_capEPBadge(cap:cap_t)->u64{
    cap.words[1] & 0xffffffffffffffffu64
}

#[inline]
pub fn cap_notification_cap_get_capNtfnBadge(cap:cap_t)->u64{
    cap.words[1] & 0xffffffffffffffffu64
}

pub fn cap_untyped_cap_get_capPtr(cap: cap_t) -> u64 {
    let mut ret = cap.words[0] & 0xffffffffffffu64;
    if ret & (1u64 << 47) != 0 {
        ret |= 0xffff000000000000u64;
    }
    ret
}

pub fn cap_untyped_cap_set_capFreeIndex(mut cap: cap_t, v64: u64) -> cap_t {
    cap.words[1] &= !0xffffffffffff0000u64;
    cap.words[1] |= (v64 << 16) & 0xffffffffffff0000u64;
    cap
}
pub fn cap_untyped_cap_get_capFreeIndex(cap: cap_t) -> u64 {
    (cap.words[1] & 0xffffffffffff0000u64) >> 16
}
pub fn cap_untyped_cap_get_capBlockSize(cap: cap_t) -> u64 {
    cap.words[1] & 0x3fu64
}
pub fn cap_untyped_cap_get_capIsDevice(cap: cap_t) -> u64 {
    (cap.words[1] & 0x40u64) >> 6
}

pub fn thread_state_get_tsType(thread_state:&thread_state_t)->u64{
    let ret:u64= thread_state.words[0] & 0xfu64;
    ret
}
pub fn thread_state_ptr_set_tsType(thread_state_ptr:&mut thread_state_t,v64:u64){
    thread_state_ptr.words[0] &= !0xfu64;
    thread_state_ptr.words[0] |= v64 & 0xf;
}

#[repr(C)]
pub struct endpoint{
    words:[u64;2]
}
pub type endpoint_t=endpoint;

#[repr(C)]
pub struct seL4_Fault{
    words:[u64;2]
}
pub type seL4_Fault_t=seL4_Fault;

pub fn seL4_Fault_get_seL4_FaultType(seL4_Fault:&seL4_Fault_t)->u64{
    seL4_Fault.words[0]&0x7u64
}
pub fn seL4_Fault_NullFault_new()->seL4_Fault_t{
    seL4_Fault_t{
        words:[0,0]
    }
}

//include/arch/x86/arch/machine/registerset.h
const CONFIG_XSAVE_SIZE:usize=512;
#[repr(C)]
struct user_fpu_state_t{
    state: [u8;CONFIG_XSAVE_SIZE]
}

const n_contextRegisters:usize=23;
#[repr(C)]
pub struct user_context_t{
    fpuState: user_fpu_state_t,
    pub registers: [word_t;n_contextRegisters]
}

//include/arch/x86/arch/machine/hardware.h
#[repr(C)]
enum vm_page_size {
    X86_SmallPage,
    X86_LargePage,
    X64_HugePage
}
type vm_page_size_t=word_t;

const seL4_PageBits:u64=12;
const seL4_LargePageBits:u64=21;
const seL4_HugePageBits:u64=30;

#[inline]
fn pageBitsForSize(pagesize:vm_page_size_t)->word_t{
    match pagesize{
        pagesize if pagesize==(vm_page_size::X86_SmallPage as u64) =>
            seL4_PageBits,
        pagesize if pagesize==(vm_page_size::X86_LargePage as u64) =>
            seL4_LargePageBits,
        pagesize if pagesize==(vm_page_size::X64_HugePage as u64) =>
            seL4_HugePageBits,
        _ => panic!("Invalid page size") //原来是fail，这里改成panic
    }
}

//include/arch/x86/arch/64/mode/object/structures.h
const seL4_PML4Bits:u64=12;
const seL4_PDPTBits:u64=12;

#[inline]
fn cap_get_modeCapSizeBits(cap:cap_t)->word_t{
    let ctag=cap_get_capType(cap);
    match ctag{
        ctag if ctag==(cap_tag_t::cap_pml4_cap as u64) =>
            seL4_PML4Bits,
        ctag if ctag==(cap_tag_t::cap_pdpt_cap as u64) =>
            seL4_PDPTBits,
        _ => 0
    }
}

#[inline]
fn cap_get_modeCapIsPhysical(cap:cap_t)->bool_t{
    let ctag=cap_get_capType(cap);
    match ctag{
        ctag if ctag==(cap_tag_t::cap_pml4_cap as u64) ||
                ctag==(cap_tag_t::cap_pdpt_cap as u64) =>
            types::_bool::r#true as u64,
        _ => types::_bool::r#false as u64
    }
}


//include/arch/x86/arch/object/structures.h
#[repr(C)]
pub struct arch_tcb_t{
    pub tcbContext: user_context_t
}

const seL4_PageTableBits:u64=12;
const seL4_PageDirBits:u64=12;
const seL4_ASIDPoolBits:u64=12;

#[inline]
pub fn cap_get_archCapSizeBits(cap:cap_t)->word_t{
    let ctag=cap_get_capType(cap);
    match ctag{
        ctag if ctag==(cap_tag_t::cap_frame_cap as u64) =>
            pageBitsForSize(cap_frame_cap_get_capFSize(cap)),
        ctag if ctag==(cap_tag_t::cap_page_table_cap as u64) =>
            seL4_PageTableBits,
        ctag if ctag==(cap_tag_t::cap_page_directory_cap as u64) =>
            seL4_PageDirBits,
        ctag if ctag==(cap_tag_t::cap_io_port_cap as u64) =>
            0,
        ctag if ctag==(cap_tag_t::cap_asid_control_cap as u64) =>
            0,
        ctag if ctag==(cap_tag_t::cap_asid_pool_cap as u64) =>
            seL4_ASIDPoolBits,
        _ => cap_get_modeCapSizeBits(cap)
    }
}

#[inline]
pub fn cap_get_archCapIsPhysical(cap:cap_t)->bool_t{
    let ctag=cap_get_capType(cap);
    match ctag{
        ctag if ctag==(cap_tag_t::cap_frame_cap as u64) ||
                ctag==(cap_tag_t::cap_page_table_cap as u64) ||
                ctag==(cap_tag_t::cap_page_directory_cap as u64) ||
                ctag==(cap_tag_t::cap_asid_pool_cap as u64) =>
            types::_bool::r#true as u64,
        ctag if ctag==(cap_tag_t::cap_io_port_cap as u64) ||
                ctag==(cap_tag_t::cap_asid_control_cap as u64) =>
            types::_bool::r#false as u64,
        _ => cap_get_modeCapIsPhysical(cap)
    }
}

#[inline]
pub fn Arch_isCapRevocable(derivedCap:cap_t,srcCap:cap_t)->bool_t{
    if cap_get_capType(derivedCap) == cap_tag_t::cap_io_port_cap as u64 {
        ( cap_get_capType(srcCap) == cap_tag_t::cap_io_port_control_cap as u64 ) as u64
    } else {
        types::_bool::r#false as u64
    }
}