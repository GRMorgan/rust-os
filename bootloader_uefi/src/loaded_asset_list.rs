use x86_64_hardware::memory::{PAGE_SIZE, PhysicalAddress};
use x86_64_hardware::memory::paging::PageFrameAllocator;
use r_efi::efi;

use crate::uefi;

#[derive(Clone, Copy)]
pub struct LoadedAsset {
    pub physical_address: u64,
    pub num_pages: usize,
    pub virtual_address: u64,    
}

pub struct LoadedAssetList {
    list_ptr: *mut LoadedAsset,
    num_pages: usize,
    num_items: usize,
}

impl LoadedAssetList {
    pub fn new(item_count: usize, system_table: uefi::SystemTableWrapper) -> Result<LoadedAssetList, efi::Status> {
        let min_mem_size = core::mem::size_of::<LoadedAsset>() * item_count;
        let mut num_pages = min_mem_size / PAGE_SIZE as usize;

        if min_mem_size % PAGE_SIZE as usize > 0 {
            num_pages += 1;
        }

        let list_ptr = system_table.boot_services().allocate_pages::<LoadedAsset>(r_efi::system::LOADER_DATA, num_pages)?;

        return Ok(LoadedAssetList {
            list_ptr: list_ptr,
            num_pages: num_pages,
            num_items: 0
        });
    }

    pub fn max_items(&self) -> usize {
        return (PAGE_SIZE as usize * self.num_pages) / core::mem::size_of::<LoadedAsset>();
    }

    /// Adds an asset to this list. Returns the index of the added item if successful.
    /// If the list is full it returns None
    pub fn add_asset(&mut self, physical_address: u64, num_pages: usize, virtual_address: u64) -> Option<usize> {
        if self.max_items() == self.num_items {
            return None;
        }
        
        let index = self.num_items;

        //This is safe as we've checked the list isn't full and we allocated sufficient memory
        //in the constructor
        unsafe {
            *(self.list_ptr.offset(index as isize)) = LoadedAsset {
                physical_address: physical_address,
                num_pages: num_pages,
                virtual_address: virtual_address,
            };
        }
        self.num_items += 1;

        return Some(index);
    }

    pub fn get_asset(&self, index: usize) -> Option<LoadedAsset> {
        if index >= self.num_items {
            return None;
        }

        //This is safe because we've checked the index isn't beyond the end of the list
        //list_ptr is guaranteed to be valid by the constructor
        unsafe { return Some(*(self.list_ptr.offset(index as isize))); }
    }

    pub fn iter(&self) -> LoadedAssetListIterator {
        LoadedAssetListIterator {
            asset_list: self,
            current_index: 0,
            max_index: self.num_items
        }
    }

    pub fn lock_list_pages(&self, allocator: &mut PageFrameAllocator) {
        allocator.lock_pages(PhysicalAddress::new(self.list_ptr as u64), self.num_pages);
    }
}

pub struct LoadedAssetListIterator<'a> {
    asset_list: &'a LoadedAssetList,
    current_index: usize,
    max_index: usize,
}

impl<'a> Iterator for LoadedAssetListIterator<'a> {
    type Item = LoadedAsset;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.max_index {
            return None;
        } else {
            let output = self.asset_list.get_asset(self.current_index);
            if output.is_some() {
                self.current_index += 1;
            }
            return output;
        }
    }
}
