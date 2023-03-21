const RSDP_V1_SIGNATURE: [u8;8] = [b'R', b'S', b'D', b' ', b'P', b'T', b'R', b' '];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RsdpV1 {
    signature: [u8;8],
    checksum: u8,
    oem_id: [u8;6],
    revision: u8,
    rsdt_physical_address: u32,
}

impl RsdpV1 {
    pub fn is_valid(&self) -> bool {
        return self.valid_signature() && self.valid_checksum();
    }

    fn valid_signature(&self) -> bool {
        self.signature == RSDP_V1_SIGNATURE
    }

    fn valid_checksum(&self) -> bool {
        let mut sum_u8: u64 = 0;

        for chr in self.signature {
            sum_u8 += chr as u64;
        }

        sum_u8 += self.checksum as u64;

        for chr in self.oem_id {
            sum_u8 += chr as u64;
        }

        sum_u8 += self.revision as u64;
        sum_u8 += (self.rsdt_physical_address & 0b1111_1111) as u64;
        sum_u8 += (self.rsdt_physical_address >> 8 & 0b1111_1111) as u64;
        sum_u8 += (self.rsdt_physical_address >> 16 & 0b1111_1111) as u64;
        sum_u8 += (self.rsdt_physical_address >> 24 & 0b1111_1111) as u64;

        return (sum_u8 & 0b1111_1111) == 0;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RsdpV2 {
    v1: RsdpV1,
    length: u32,
    xdst_physical_address: u64,
    extended_checksum: u8,
    reserved: [u8;3],
}

impl RsdpV2 {
    pub fn is_valid(&self) -> bool {
        return self.v1.is_valid() && self.valid_checksum();
    }

    fn valid_checksum(&self) -> bool {
        let mut sum_u8: u64 = 0;

        sum_u8 += (self.length & 0b1111_1111) as u64;
        sum_u8 += (self.length >> 8 & 0b1111_1111) as u64;
        sum_u8 += (self.length >> 16 & 0b1111_1111) as u64;
        sum_u8 += (self.length >> 24 & 0b1111_1111) as u64;
        
        sum_u8 += (self.xdst_physical_address & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 8 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 16 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 24 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 32 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 40 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 48 & 0b1111_1111) as u64;
        sum_u8 += (self.xdst_physical_address >> 56 & 0b1111_1111) as u64;

        sum_u8 += self.extended_checksum as u64;

        for chr in self.reserved {
            sum_u8 += chr as u64;
        }

        return (sum_u8 & 0b1111_1111) == 0;
    }
}