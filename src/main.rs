use std::collections::HashMap;

use bit_field::BitField;
use rand::Rng;

const PAGE_SIZE:usize = 4096; // PAGE_ENTRY_SIZE * PAGE_TABLE_ENTRIES
const PAGE_TABLE_ENTRIES:usize = 512;
const USER_SPACE_START_ADDRESS:usize = 0x0000_0000_0000_0000;
const USER_SPACE_END_ADDRESS:usize = 0x0000_7FFF_FFFF_FFFF;
const RECURSIVE_ENTRY : usize = 511;
const PAGE_ENTRY_SIZE: usize = 8;  //8 bytes


#[repr(transparent)]
#[derive(Clone, Copy)]
struct PageTableEntry(u64);


impl PageTableEntry{

    fn new(phys_addr:u64, writable:bool,no_execute:bool,present:bool)->Self{
        assert!(phys_addr as usize % PAGE_SIZE ==0 , "Physical address must be aligned with the page size");
 
          let mut entry = PageTableEntry(0);
          entry.0.set_bit(0, present);
          entry.0.set_bit(1, writable);
          entry.0.set_bit(63, no_execute);
        // extract bits 12-51 
        // let  phys_addr_extract  = phys_addr & 0x000F_FFFF_FFFF_F000; 
        // add them to the entry
        // entry.0.set_bits(12..51,phys_addr_extract);
        entry.0 |= phys_addr & 0x000F_FFFF_FFFF_F000;

        entry

    }

    fn is_present(&self)->bool{
        self.0.get_bit(0)
    }

    fn get_phys_address(&self)->u64{
        self.0 & 0x000F_FFFF_FFFF_F000
    }


}

struct PageTable{
    entries:[PageTableEntry;PAGE_TABLE_ENTRIES]
}

impl PageTable{
    fn new()->Self{
        PageTable{ entries:[PageTableEntry(0);PAGE_TABLE_ENTRIES]}
    }

    fn add_entry(&mut self,index:usize,entry:PageTableEntry){
        // prblm: can overwrite other entries 
        self.entries[index] = entry;

    }

    fn get_entry(&self,index:usize)->&PageTableEntry{
         &self.entries[index]
    }
}

struct PhysicalMemory{
    page_tables: HashMap<u64,PageTable>
}
impl PhysicalMemory{
    fn new()->Self{
        PhysicalMemory{page_tables: HashMap::new()}
    }
    fn store_page_table(&mut self, phys_addr:u64 ,page_table:PageTable){
        self.page_tables.insert(phys_addr, page_table);
    }

    fn get_page_table(&self, phys_addr:u64)->Option<&PageTable>{
        self.page_tables.get(&phys_addr)
    }

    fn get_mut_page_table(&mut self, phys_addr:u64)->Option<&mut PageTable>{
        self.page_tables.get_mut(&phys_addr)
    }
}

struct FrameAllocator{
    next_frame: usize,
    total_frames:usize
}

impl FrameAllocator{
    fn new(start:usize,total_frames:usize)->Self{
        FrameAllocator { next_frame: start, total_frames: total_frames }

    }

    fn allocate_frame(&mut self)->Option<usize>{
        if self.next_frame < self.total_frames{
            // calculate the frame block start address
            let frame_address = self.next_frame * PAGE_SIZE;
            self.next_frame += 1;
            return Some(frame_address);

        }
        None
    }

    
}

fn check_address_alignment(addr:usize, page_size:usize)->u64{
    let alignment = addr as usize % page_size;
     if alignment != 0 {
       return (addr & !(page_size - 1)) as u64;
     }
     return addr as u64;

}
fn generate_virtual_address()->u64{
      let mut rand_gen = rand::thread_rng();
    let address = rand_gen.gen_range(USER_SPACE_START_ADDRESS..USER_SPACE_END_ADDRESS);
    let aligned_addr =  check_address_alignment(address, PAGE_SIZE);
    aligned_addr
} 

fn virt_addr_indices(virt_addr:u64)->(usize,usize,usize,usize){
     let virt_addr = virt_addr as usize;
    //  64 bit address structure
    //  0x_sign_pml4idx_pdtpidx_pdtidx_ptidx_offset
    //  | 63-48 (unused) | 47-39 (PML4) | 38-30 (PDP) | 29-21 (PD) | 20-12 (PT) | 11-0 (Offset) |
    //  each index is 9 bits use a 9 bits mask = 0x1FF == 0b1_1111_1111
    let idx_mask = 0x1FF;
    let offset_mask = 0xFFF;
    let pml4_idx = (virt_addr >>39) & idx_mask;
    let pdpt_idx = (virt_addr >> 30 ) & idx_mask;
    let pdt_idx = (virt_addr >> 21) & idx_mask;
    let pt_idx = (virt_addr >> 12) & idx_mask;

    let offset = virt_addr & offset_mask;

    (pml4_idx,pdpt_idx,pdt_idx,pt_idx)
}

fn pml4_virtual_access_addr( pml4_idx: usize)->usize{
     (RECURSIVE_ENTRY << 39) | (RECURSIVE_ENTRY << 30) | (RECURSIVE_ENTRY << 21) | (RECURSIVE_ENTRY << 12)| (pml4_idx * PAGE_ENTRY_SIZE)

}

fn pdtp_virtual_access_addr(pml4_idx: usize,pdpt_idx:usize)->usize{
    (RECURSIVE_ENTRY << 39)| (RECURSIVE_ENTRY << 30) | (RECURSIVE_ENTRY << 21) | ( pml4_idx << 12) | (pdpt_idx * PAGE_ENTRY_SIZE)
}

fn pdt_virtual_access_addr(pml4_idx: usize,pdpt_idx:usize,pdt_idx:usize)->usize{
    (RECURSIVE_ENTRY << 39) | (RECURSIVE_ENTRY << 30)| (pml4_idx << 21) | (pdpt_idx << 12) | (pdt_idx * PAGE_ENTRY_SIZE)
}

fn pt_virtual_access_addr(pml4_idx: usize,pdpt_idx:usize,pdt_idx:usize,pt_idx:usize)->usize{
    (RECURSIVE_ENTRY << 39) | (pml4_idx << 30) | (pdpt_idx << 21)| (pdt_idx << 12)|(pt_idx * PAGE_ENTRY_SIZE)

}

// translate virtual to physical address using the recurisive entry
fn translate_virtual_to_physical_address(virt_addr:u64,phys_mem:&PhysicalMemory,frame_allocator:&FrameAllocator)->u64{
    // break down virtual address to indices into PML4, PDTP, PDT,PT
    let (pml4_idx,pdpt_idx,pdt_idx,pt_idx) = virt_addr_indices(virt_addr);

    // create virtual addresses to access page tables using the indices
    let pml4_entry_address = pml4_virtual_access_addr(pml4_idx);





    // if a page table is not created create it 
    // save page table in memory
    // return the physical address of the virtual address
}


fn main(){
    // initalize memory and frame allocator

  let  frame_allocator = FrameAllocator::new(1,10);
  let physicalMemory = PhysicalMemory::new();
  

    println!("{:?}","working");


}
/*
 virtual address:h
      1. extract indixes into page tables - index gives the entry we want
      2. In recursive paging construct an address of the table address of the table we want to access using the recursive entry, 
      but if traversing using the indices we will use the there is no need to construct the address
      3.    
     
*/