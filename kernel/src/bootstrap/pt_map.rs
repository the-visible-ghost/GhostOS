use crate::bootstrap::pt_arena::PtArena;
use crate::memory::paging::PageTable;
use crate::memory::paging::{pd_index, pdpt_index, pml4_index, pt_index};

#[inline(always)]
pub fn linear(
    pml4: &mut PageTable,
    pt_arena: &mut PtArena,
    virt: *mut u8,
    phys: *mut u8,
    count: usize,
    flags: u32,
) {
    let mut index = 0;
    while index < count {
        let addr = unsafe { virt.add(index << 12) };

        let pml4ei = pml4_index(addr as u64);
        let pml4e = pml4.get_entry(pml4ei);
        if !pml4e.is_present() {
            let new_pdpt = pt_arena.allocate();
            new_pdpt.clear_all();
            pml4e.set_present();
            pml4e.set_table(new_pdpt);
        }

        let pdpt = pml4e.get_table_mut();
        let pdptei = pdpt_index(addr as u64);
        let pdpte = pdpt.get_entry(pdptei);
        if !pdpte.is_present() {
            let new_pd = pt_arena.allocate();
            new_pd.clear_all();
            pdpte.set_present();
            pdpte.set_table(new_pd);
        }

        let pd = pdpte.get_table_mut();
        let pdei = pd_index(addr as u64);
        let pde = pd.get_entry(pdei);
        if !pde.is_present() {
            let new_pt = pt_arena.allocate();
            new_pt.clear_all();
            pde.set_present();
            pde.set_table(new_pt);
        }

        let pt = pde.get_table_mut();
        let ptei = pt_index(addr as u64);
        let pte = pt.get_entry(ptei);
        pte.set_present();
        pte.set_page(unsafe { phys.add(index << 12) });

        if flags & 0x01 == 0 {
            pte.set_no_execute();
        }
        if flags & 0x02 != 0 {
            pte.set_read_write();
        }

        index += 1;
    }
}
