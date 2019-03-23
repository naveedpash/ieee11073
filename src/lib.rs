// define types
type OidType = u16;
type PrivateOid = u16;
type Handle = u16;
type InstNumber = u16;
type MdsContext = u16;

struct GlbHandle {
    context_id: MdsContext,
    handle: Handle,
}

// define Managed Object ID
struct ManagedObjectId {
    m_obj_class: OidType,
    m_obj_inst: GlbHandle,
}

// define nomenclature partitions
enum NomPartition {
    nom_part_unspec = 0u16,
    nom_part_obj = 1u16,
    nom_part_metric = 2u16,
    nom_part_alert = 3u16,
    nom_part_dim = 4u16,
    nom_part_vattr = 5u16,
    nom_part_pgrp = 6u16,
    nom_part_sites = 7u16,
    nom_part_infrastruct = 8u16,
    nom_part_fef = 9u16,
    nom_part_ecg_extn = 10u16,
    nom_part_icdo = 11u16,
    nom_part_phddm = 128u16,
    nom_part_hf = 129u16,
    nom_part_ageind = 130u16,
    nom_part_returncodes = 255u16,
    nom_part_ext_nom = 256u16,
    nom_part_settings = 258u16,
    nom_part_priv = 1024u16,
}

//define Type ID
struct Type {
    partition: NomPartition,
    code: OidType,
}

//define Attribute value assertion
struct AVA_Type {
    attribute_id: OidType,
    attribute_value: any,
}

type AttributeList = Array<AVA_Type>;
type AttributeIdList = Array<OidType>;

//define Time Types
type RelativeTime = u32;
type HighSerRelativeTime = &str;
struct AbsoluteTime {
    century: u8,
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    sec_fractions: u8,
}

struct Date {
    century: u8,
    year: u8,
    month: u8,
    day: u8,
}

//define Operational State
enum OperationStateState {
    disabled = 0u16,
    enabled = 1u16,
    notAvailable = 2u16,
}

//define Administrative State
enum AdministrativeState {
    locked = 0u16,
    unlocked = 1u16,
    shuttingDown = 2u16,
}

//define Color Data
// -- 3 bit representation --
enum SimpleColour {
    col_black = 0b000,
    col_red = 0b100,
    col_green = 0b010,
    col_yellow = 0b110,
    col_blue = 0b001,
    col_magenta = 0b101,
    col_cyan = 0b011,
    col_white = 0b111,
}

// -- U16 representation --
enum SimpleColour {
    col_black = 0u16,
    col_red = 1u16,
    col_green = 2u16,
    col_yellow = 3u16,
    col_blue = 4u16,
    col_magenta = 5u16,
    col_cyan = 6u16,
    col_white = 7u16,
}

//define Locale Data
enum CharSet {}
