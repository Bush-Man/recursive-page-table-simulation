use bit_field::BitField;

const PAGE_SIZE:usize = 4096;
const PAGE_TABLE_ENTRIES:usize = 512;


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

fn main(){
    
}
