#[derive(PartialEq, Debug)]
pub enum SignatureType {
    APIC,
    BERT,
    BGRT,
    CCEL,
    CPEP,
    DSDT,
    ECDT,
    EINJ,
    ERST,
    FACP,
    FACS,
    FPDT,
    GTDT,
    HEST,
    MISC,
    MSCT,
    MPST,
    NFIT,
    OEMx,
    PCCT,
    PHAT,
    PMTT,
    PPTT,
    PSDT,
    RASF,
    RAS2,
    RSDT,
    SBST,
    SDEV,
    SLIT,
    SRAT,
    SSDT,
    SVKL,
    XSDT,
    AEST,
    AGDI,
    APMT,
    BDAT,
    BOOT,
    CEDT,
    CSRT,
    DBGP,
    DBG2,
    DMAR,
    DRTM,
    DTPR,
    ETDT,
    HPET,
    IBFT,
    IERS,
    IORT,
    IVRS,
    KEYP,
    LPIT,
    MCFG,
    MCHI,
    MHSP,
    MPAM,
    MSDM,
    NBFT,
    PRMT,
    RGRT,
    SDEI,
    SLIC,
    SPCR,
    SPMI,
    STAO,
    SWFT,
    TCPA,
    TPM2,
    UEFI,
    WAET,
    WDAT,
    WDDT,
    WDRT,
    WPBT,
    WSMT,
    XENV,
    Unknown,
}

const APIC_SIGNATURE: [u8;4] = [b'A', b'P', b'I', b'C'];
const BERT_SIGNATURE: [u8;4] = [b'B', b'E', b'R', b'T'];
const BGRT_SIGNATURE: [u8;4] = [b'B', b'G', b'R', b'T'];
const CCEL_SIGNATURE: [u8;4] = [b'C', b'C', b'E', b'L'];
const CPEP_SIGNATURE: [u8;4] = [b'C', b'P', b'E', b'P'];
const DSDT_SIGNATURE: [u8;4] = [b'D', b'S', b'D', b'T'];
const ECDT_SIGNATURE: [u8;4] = [b'E', b'C', b'D', b'T'];
const EINJ_SIGNATURE: [u8;4] = [b'E', b'I', b'N', b'J'];
const ERST_SIGNATURE: [u8;4] = [b'E', b'R', b'S', b'T'];
const FACP_SIGNATURE: [u8;4] = [b'F', b'A', b'C', b'P'];
const FACS_SIGNATURE: [u8;4] = [b'F', b'A', b'C', b'S'];
const FPDT_SIGNATURE: [u8;4] = [b'F', b'P', b'D', b'T'];
const GTDT_SIGNATURE: [u8;4] = [b'G', b'T', b'D', b'T'];
const HEST_SIGNATURE: [u8;4] = [b'H', b'E', b'S', b'T'];
const MISC_SIGNATURE: [u8;4] = [b'M', b'I', b'S', b'C'];
const MSCT_SIGNATURE: [u8;4] = [b'M', b'S', b'C', b'T'];
const MPST_SIGNATURE: [u8;4] = [b'M', b'P', b'S', b'T'];
const NFIT_SIGNATURE: [u8;4] = [b'N', b'F', b'I', b'T'];
const OEMX_SIGNATURE: [u8;4] = [b'O', b'E', b'M', b'x'];
const PCCT_SIGNATURE: [u8;4] = [b'P', b'C', b'C', b'T'];
const PHAT_SIGNATURE: [u8;4] = [b'P', b'H', b'A', b'T'];
const PMTT_SIGNATURE: [u8;4] = [b'P', b'M', b'T', b'T'];
const PPTT_SIGNATURE: [u8;4] = [b'P', b'P', b'T', b'T'];
const PSDT_SIGNATURE: [u8;4] = [b'P', b'S', b'D', b'T'];
const RASF_SIGNATURE: [u8;4] = [b'R', b'A', b'S', b'F'];
const RAS2_SIGNATURE: [u8;4] = [b'R', b'A', b'S', b'2'];
const RSDT_SIGNATURE: [u8;4] = [b'R', b'S', b'D', b'T'];
const SBST_SIGNATURE: [u8;4] = [b'S', b'B', b'S', b'T'];
const SDEV_SIGNATURE: [u8;4] = [b'S', b'D', b'E', b'V'];
const SLIT_SIGNATURE: [u8;4] = [b'S', b'L', b'I', b'T'];
const SRAT_SIGNATURE: [u8;4] = [b'S', b'R', b'A', b'T'];
const SSDT_SIGNATURE: [u8;4] = [b'S', b'S', b'D', b'T'];
const SVKL_SIGNATURE: [u8;4] = [b'S', b'V', b'K', b'L'];
const XSDT_SIGNATURE: [u8;4] = [b'X', b'S', b'D', b'T'];
const AEST_SIGNATURE: [u8;4] = [b'A', b'E', b'S', b'T'];
const AGDI_SIGNATURE: [u8;4] = [b'A', b'G', b'D', b'I'];
const APMT_SIGNATURE: [u8;4] = [b'A', b'P', b'M', b'T'];
const BDAT_SIGNATURE: [u8;4] = [b'B', b'D', b'A', b'T'];
const BOOT_SIGNATURE: [u8;4] = [b'B', b'O', b'O', b'T'];
const CEDT_SIGNATURE: [u8;4] = [b'C', b'E', b'D', b'T'];
const CSRT_SIGNATURE: [u8;4] = [b'C', b'S', b'R', b'T'];
const DBGP_SIGNATURE: [u8;4] = [b'D', b'B', b'G', b'P'];
const DBG2_SIGNATURE: [u8;4] = [b'D', b'B', b'G', b'2'];
const DMAR_SIGNATURE: [u8;4] = [b'D', b'M', b'A', b'R'];
const DRTM_SIGNATURE: [u8;4] = [b'D', b'R', b'T', b'M'];
const DTPR_SIGNATURE: [u8;4] = [b'D', b'T', b'P', b'R'];
const ETDT_SIGNATURE: [u8;4] = [b'E', b'T', b'D', b'T'];
const HPET_SIGNATURE: [u8;4] = [b'H', b'P', b'E', b'T'];
const IBFT_SIGNATURE: [u8;4] = [b'I', b'B', b'F', b'T'];
const IERS_SIGNATURE: [u8;4] = [b'I', b'E', b'R', b'S'];
const IORT_SIGNATURE: [u8;4] = [b'I', b'O', b'R', b'T'];
const IVRS_SIGNATURE: [u8;4] = [b'I', b'V', b'R', b'S'];
const KEYP_SIGNATURE: [u8;4] = [b'K', b'E', b'Y', b'P'];
const LPIT_SIGNATURE: [u8;4] = [b'L', b'P', b'I', b'T'];
const MCFG_SIGNATURE: [u8;4] = [b'M', b'C', b'F', b'G'];
const MCHI_SIGNATURE: [u8;4] = [b'M', b'C', b'H', b'I'];
const MHSP_SIGNATURE: [u8;4] = [b'M', b'H', b'S', b'P'];
const MPAM_SIGNATURE: [u8;4] = [b'M', b'P', b'A', b'M'];
const MSDM_SIGNATURE: [u8;4] = [b'M', b'S', b'D', b'M'];
const NBFT_SIGNATURE: [u8;4] = [b'N', b'B', b'F', b'T'];
const PRMT_SIGNATURE: [u8;4] = [b'P', b'R', b'M', b'T'];
const RGRT_SIGNATURE: [u8;4] = [b'R', b'G', b'R', b'T'];
const SDEI_SIGNATURE: [u8;4] = [b'S', b'D', b'E', b'I'];
const SLIC_SIGNATURE: [u8;4] = [b'S', b'L', b'I', b'C'];
const SPCR_SIGNATURE: [u8;4] = [b'S', b'P', b'C', b'R'];
const SPMI_SIGNATURE: [u8;4] = [b'S', b'P', b'M', b'I'];
const STAO_SIGNATURE: [u8;4] = [b'S', b'T', b'A', b'O'];
const SWFT_SIGNATURE: [u8;4] = [b'S', b'W', b'F', b'T'];
const TCPA_SIGNATURE: [u8;4] = [b'T', b'C', b'P', b'A'];
const TPM2_SIGNATURE: [u8;4] = [b'T', b'P', b'M', b'2'];
const UEFI_SIGNATURE: [u8;4] = [b'U', b'E', b'F', b'I'];
const WAET_SIGNATURE: [u8;4] = [b'W', b'A', b'E', b'T'];
const WDAT_SIGNATURE: [u8;4] = [b'W', b'D', b'A', b'T'];
const WDDT_SIGNATURE: [u8;4] = [b'W', b'D', b'D', b'T'];
const WDRT_SIGNATURE: [u8;4] = [b'W', b'D', b'R', b'T'];
const WPBT_SIGNATURE: [u8;4] = [b'W', b'P', b'B', b'T'];
const WSMT_SIGNATURE: [u8;4] = [b'W', b'S', b'M', b'T'];
const XENV_SIGNATURE: [u8;4] = [b'X', b'E', b'N', b'V'];

#[repr(C)]
pub struct SystemDescriptionTableHeader {
    signature: [u8;4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8;6],
    oem_table_id: [u8;8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl SystemDescriptionTableHeader {
    pub fn length(&self) -> u32 {
        return self.length;
    }
}

pub struct SystemDescriptionTable {
    sdt_ptr: *mut SystemDescriptionTableHeader,
    mem_offset: u64,
}

impl SystemDescriptionTable {
    /// Creates a new SystemDescriptionTable that wraps an SystemDescriptionTableHeader pointer. We
    /// use a wrapper because the data can extend off beyond the last entry in the internal struct.
    /// So if we want to implement Copy for this it must contain the pointer internally.
    /// 
    /// Additionally we don't know precisely what kind of SDT this is. Potentially we can allow for
    /// a conversion to a more accurate vendor specific SDT here.
    /// 
    /// ## Safety
    /// 
    /// This is unsafe because we cannot know the address and offset are valid. The address
    /// must come from the RSDT and the offset from the virtual memory management logic of
    /// the OS
    pub unsafe fn new(physical_address: u64, offset: u64) -> SystemDescriptionTable {
        let virtual_address = physical_address + offset;

        return SystemDescriptionTable { 
            sdt_ptr: virtual_address as *mut SystemDescriptionTableHeader,
            mem_offset: offset,
        }
    }

    pub fn get_signature_array(&self) -> [u8;4] {
        unsafe { (*self.sdt_ptr).signature }
    }

    pub fn get_signature(&self) -> SignatureType {
        match self.get_signature_array() {
            APIC_SIGNATURE => SignatureType::APIC,
            BERT_SIGNATURE => SignatureType::BERT,
            BGRT_SIGNATURE => SignatureType::BGRT,
            CCEL_SIGNATURE => SignatureType::CCEL,
            CPEP_SIGNATURE => SignatureType::CPEP,
            DSDT_SIGNATURE => SignatureType::DSDT,
            ECDT_SIGNATURE => SignatureType::ECDT,
            EINJ_SIGNATURE => SignatureType::EINJ,
            ERST_SIGNATURE => SignatureType::ERST,
            FACP_SIGNATURE => SignatureType::FACP,
            FACS_SIGNATURE => SignatureType::FACS,
            FPDT_SIGNATURE => SignatureType::FPDT,
            GTDT_SIGNATURE => SignatureType::GTDT,
            HEST_SIGNATURE => SignatureType::HEST,
            MISC_SIGNATURE => SignatureType::MISC,
            MSCT_SIGNATURE => SignatureType::MSCT,
            MPST_SIGNATURE => SignatureType::MPST,
            NFIT_SIGNATURE => SignatureType::NFIT,
            OEMX_SIGNATURE => SignatureType::OEMx,
            PCCT_SIGNATURE => SignatureType::PCCT,
            PHAT_SIGNATURE => SignatureType::PHAT,
            PMTT_SIGNATURE => SignatureType::PMTT,
            PPTT_SIGNATURE => SignatureType::PPTT,
            PSDT_SIGNATURE => SignatureType::PSDT,
            RASF_SIGNATURE => SignatureType::RASF,
            RAS2_SIGNATURE => SignatureType::RAS2,
            RSDT_SIGNATURE => SignatureType::RSDT,
            SBST_SIGNATURE => SignatureType::SBST,
            SDEV_SIGNATURE => SignatureType::SDEV,
            SLIT_SIGNATURE => SignatureType::SLIT,
            SRAT_SIGNATURE => SignatureType::SRAT,
            SSDT_SIGNATURE => SignatureType::SSDT,
            SVKL_SIGNATURE => SignatureType::SVKL,
            XSDT_SIGNATURE => SignatureType::XSDT,
            AEST_SIGNATURE => SignatureType::AEST,
            AGDI_SIGNATURE => SignatureType::AGDI,
            APMT_SIGNATURE => SignatureType::APMT,
            BDAT_SIGNATURE => SignatureType::BDAT,
            BOOT_SIGNATURE => SignatureType::BOOT,
            CEDT_SIGNATURE => SignatureType::CEDT,
            CSRT_SIGNATURE => SignatureType::CSRT,
            DBGP_SIGNATURE => SignatureType::DBGP,
            DBG2_SIGNATURE => SignatureType::DBG2,
            DMAR_SIGNATURE => SignatureType::DMAR,
            DRTM_SIGNATURE => SignatureType::DRTM,
            DTPR_SIGNATURE => SignatureType::DTPR,
            ETDT_SIGNATURE => SignatureType::ETDT,
            HPET_SIGNATURE => SignatureType::HPET,
            IBFT_SIGNATURE => SignatureType::IBFT,
            IERS_SIGNATURE => SignatureType::IERS,
            IORT_SIGNATURE => SignatureType::IORT,
            IVRS_SIGNATURE => SignatureType::IVRS,
            KEYP_SIGNATURE => SignatureType::KEYP,
            LPIT_SIGNATURE => SignatureType::LPIT,
            MCFG_SIGNATURE => SignatureType::MCFG,
            MCHI_SIGNATURE => SignatureType::MCHI,
            MHSP_SIGNATURE => SignatureType::MHSP,
            MPAM_SIGNATURE => SignatureType::MPAM,
            MSDM_SIGNATURE => SignatureType::MSDM,
            NBFT_SIGNATURE => SignatureType::NBFT,
            PRMT_SIGNATURE => SignatureType::PRMT,
            RGRT_SIGNATURE => SignatureType::RGRT,
            SDEI_SIGNATURE => SignatureType::SDEI,
            SLIC_SIGNATURE => SignatureType::SLIC,
            SPCR_SIGNATURE => SignatureType::SPCR,
            SPMI_SIGNATURE => SignatureType::SPMI,
            STAO_SIGNATURE => SignatureType::STAO,
            SWFT_SIGNATURE => SignatureType::SWFT,
            TCPA_SIGNATURE => SignatureType::TCPA,
            TPM2_SIGNATURE => SignatureType::TPM2,
            UEFI_SIGNATURE => SignatureType::UEFI,
            WAET_SIGNATURE => SignatureType::WAET,
            WDAT_SIGNATURE => SignatureType::WDAT,
            WDDT_SIGNATURE => SignatureType::WDDT,
            WDRT_SIGNATURE => SignatureType::WDRT,
            WPBT_SIGNATURE => SignatureType::WPBT,
            WSMT_SIGNATURE => SignatureType::WSMT,
            XENV_SIGNATURE => SignatureType::XENV,
            _ => SignatureType::Unknown,
        }
    }
}