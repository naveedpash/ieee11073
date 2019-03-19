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
type NomPartition = u16;
const nom_part_unspec: NomPartition = 0;
const nom_part_obj: NomPartition = 1;
const nom_part_metric: NomPartition = 2;
const nom_part_alert: NomPartition = 3;
const nom_part_dim: NomPartition = 4;
const nom_part_vattr: NomPartition = 5;
const nom_part_pgrp: NomPartition = 6;
const nom_part_sites: NomPartition = 7;
const nom_part_infrastruct: NomPartition = 8;
const nom_part_fef: NomPartition = 9;
const nom_part_ecg_extn: NomPartition = 10;
const nom_part_icdo: NomPartition = 11;
const nom_part_phddm: NomPartition = 128;
const nom_part_hf: NomPartition = 129;
const nom_part_ageind: NomPartition = 130;
const nom_part_returncodes: NomPartition = 255;
const nom_part_ext_nom: NomPartition = 256;
const nom_part_settings: NomPartition = 258;
const nom_part_priv: NomPartition = 1024;

//define Type ID
struct Type {
    partition: NomPartition,
    code: OidType,
}
