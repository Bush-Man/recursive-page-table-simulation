use std::string;

use bit_field::BitField;


const PAGE_SIZE:usize = 4096;
const PAGE_TABLE_ENTRIES :usize= 512;

/*
Bit(s)	Name	                Meaning
0	    present	                the page is currently in memory
1	    writable	            it’s allowed to write to this page
2	    user accessible	        if not set, only kernel mode code can access this page
3	    write-through caching	writes go directly to memory
4	    disable cache	        no cache is used for this page
5	    accessed	            the CPU sets this bit when this page is used
6	    dirty	                the CPU sets this bit when a write to this page occurs
7	    huge page/null	        must be 0 in P1 and P4, creates a 1 GiB page in P3, creates a 2 MiB page in P2
8	    global	                page isn’t flushed from caches on address space switch (PGE bit of CR4 register must be set)
9-11	available	            can be used freely by the OS
12-51	physical address	    the page aligned 52bit physical address of the frame or the next page table
52-62	available	            can be used freely by the OS
63	    no execute	            forbid executing code on this page (the NXE bit in the EFER register must be set)

*/

#[repr(transparent)]
#[derive(Clone, Copy)]
struct PageTableEntry(u64);

impl PageTableEntry{
     fn new(phys_addr:usize,present:bool,writable:bool,executable:bool)->Self{
        let mut entry:u64 = 0 as u64;
        entry.set_bit(0, present);
        entry.set_bit(1, writable);
        entry.set_bit(63, executable);
        /*
         0b - binary format
         e  - executable bit
         w  - writable bit
         p  - present bit
         */

        // upto now entry address(64 bits) in binary is something like this : 0be00000000000000000000000000000000000000000000000000000000000wp
        // we need to add the pysical address between bits 52 - 12 in the binary without overwriting any existing bits 
        // combining two binaries so we use logic OR, why ? OR combines without overwriting what existed  
        // combining both addresses should be same data type
        entry |= phys_addr as u64;
        PageTableEntry(entry)
    }

fn is_present(&self)->bool{
    self.0.get_bit(0)
}

// extract bits 12-51 from the page entry we use logic & 
fn get_physical_address(&self)->usize{
    (self.0 & 0x000F_FFFF_FFFF_FF00) as usize
}
// is the program allowed to use that page to read,write or execute things in physical memory ?
fn get_permissions(&self)->&str{
    let write_permission = self.0.get_bit(1);
    let read_permission = self.0.get_bit(2);
    let execute_permission = self.0.get_bit(63);

    

    if write_permission {
        return "writable";
    }else if read_permission {
        return "read only";
    }else if execute_permission {
        return "executable";
    }else{
        return "no permission default is read only";
    }

}

}
#[repr(align(4096))]
struct PageTable{
    entries: [PageTableEntry;PAGE_TABLE_ENTRIES]
}

impl PageTable{
    fn new()->Self{
        PageTable { entries:[PageTableEntry(0);PAGE_TABLE_ENTRIES] }
    }

    fn set_entry(&mut self, index:usize,entry:PageTableEntry){
        self.entries[index] = entry;
    }

    fn get_entry(&self,index:usize)->&PageTableEntry{
        &self.entries[index]
    }
}

 fn main(){

 }