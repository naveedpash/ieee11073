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
    nom_part_unspec = 0 as u16,
    nom_part_obj = 1 as u16,
    nom_part_metric = 2 as u16,
    nom_part_alert = 3 as u16,
    nom_part_dim = 4 as u16,
    nom_part_vattr = 5 as u16,
    nom_part_pgrp = 6 as u16,
    nom_part_sites = 7 as u16,
    nom_part_infrastruct = 8 as u16,
    nom_part_fef = 9 as u16,
    nom_part_ecg_extn = 10 as u16,
    nom_part_icdo = 11 as u16,
    nom_part_phddm = 128 as u16,
    nom_part_hf = 129 as u16,
    nom_part_ageind = 130 as u16,
    nom_part_returncodes = 255 as u16,
    nom_part_ext_nom = 256 as u16,
    nom_part_settings = 258 as u16,
    nom_part_priv = 1024 as u16,
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
    disabled = 0 as u16,
    enabled = 1 as u16,
    notAvailable = 2 as u16,
}

//define Administrative State
enum AdministrativeState {
    locked = 0 as u16,
    unlocked = 1 as u16,
    shuttingDown = 2 as u16,
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
    col_black = 0 as u16,
    col_red = 1 as u16,
    col_green = 2 as u16,
    col_yellow = 3 as u16,
    col_blue = 4 as u16,
    col_magenta = 5 as u16,
    col_cyan = 6 as u16,
    col_white = 7 as u16,
}

//define Locale Data
enum Language {
    Abkhazian = 0x61620000 as u32,
    Afar = 0x61610000 as u32,
    Afrikaans = 0x61660000 as u32,
    Akan = 0x616B0000 as u32,
    Albanian = 0x73710000 as u32,
    Amharic = 0x616D0000 as u32,
    Arabic = 0x61720000 as u32,
    Aragonese = 0x616E0000 as u32,
    Armenian = 0x68790000 as u32,
    Assamese = 0x61730000 as u32,
    Avaric = 0x61760000 as u32,
    Avestan = 0x61650000 as u32,
    Aymara = 0x61790000 as u32,
    Azerbaijani = 0x617A0000 as u32,
    Bambara = 0x626D0000 as u32,
    Bashkir = 0x62610000 as u32,
    Basque = 0x65750000 as u32,
    Belarusian = 0x62650000 as u32,
    Bengali__Bangla_ = 0x626E0000 as u32,
    Bihari = 0x62680000 as u32,
    Bislama = 0x62690000 as u32,
    Bosnian = 0x62730000 as u32,
    Breton = 0x62720000 as u32,
    Bulgarian = 0x62670000 as u32,
    Burmese = 0x6D790000 as u32,
    Catalan = 0x63610000 as u32,
    Chamorro = 0x63680000 as u32,
    Chechen = 0x63650000 as u32,
    Chichewa__Chewa__Nyanja = 0x6E790000 as u32,
    Chinese = 0x7A680000 as u32,
    Chuvash = 0x63760000 as u32,
    Cornish = 0x6B770000 as u32,
    Corsican = 0x636F0000 as u32,
    Cree = 0x63720000 as u32,
    Croatian = 0x68720000 as u32,
    Czech = 0x63730000 as u32,
    Danish = 0x64610000 as u32,
    Divehi__Dhivehi__Maldivian = 0x64760000 as u32,
    Dutch = 0x6E6C0000 as u32,
    Dzongkha = 0x647A0000 as u32,
    English = 0x656E0000 as u32,
    Esperanto = 0x656F0000 as u32,
    Estonian = 0x65740000 as u32,
    Ewe = 0x65650000 as u32,
    Faroese = 0x666F0000 as u32,
    Fijian = 0x666A0000 as u32,
    Finnish = 0x66690000 as u32,
    French = 0x66720000 as u32,
    Fula__Fulah__Pulaar__Pular = 0x66660000 as u32,
    Galician = 0x676C0000 as u32,
    Gaelic__Scottish_ = 0x67640000 as u32,
    Gaelic__Manx_ = 0x67760000 as u32,
    Georgian = 0x6B610000 as u32,
    German = 0x64650000 as u32,
    Greek = 0x656C0000 as u32,
    Greenlandic = 0x6B6C0000 as u32,
    Guarani = 0x676E0000 as u32,
    Gujarati = 0x67750000 as u32,
    Haitian_Creole = 0x68740000 as u32,
    Hausa = 0x68610000 as u32,
    Hebrew = 0x68650000 as u32,
    Herero = 0x687A0000 as u32,
    Hindi = 0x68690000 as u32,
    Hiri_Motu = 0x686F0000 as u32,
    Hungarian = 0x68750000 as u32,
    Icelandic = 0x69730000 as u32,
    Ido = 0x696F0000 as u32,
    Igbo = 0x69670000 as u32,
    Indonesian = 0x69640000 as u32,
    Interlingua = 0x69610000 as u32,
    Interlingue = 0x69650000 as u32,
    Inuktitut = 0x69750000 as u32,
    Inupiak = 0x696B0000 as u32,
    Irish = 0x67610000 as u32,
    Italian = 0x69740000 as u32,
    Japanese = 0x6A610000 as u32,
    Javanese = 0x6A760000 as u32,
    Kalaallisut__Greenlandic = 0x6B6C0000 as u32,
    Kannada = 0x6B6E0000 as u32,
    Kanuri = 0x6B720000 as u32,
    Kashmiri = 0x6B730000 as u32,
    Kazakh = 0x6B6B0000 as u32,
    Khmer = 0x6B6D0000 as u32,
    Kikuyu = 0x6B690000 as u32,
    Kinyarwanda__Rwanda_ = 0x72770000 as u32,
    Kirundi = 0x726E0000 as u32,
    Kyrgyz = 0x6B790000 as u32,
    Komi = 0x6B760000 as u32,
    Kongo = 0x6B670000 as u32,
    Korean = 0x6B6F0000 as u32,
    Kurdish = 0x6B750000 as u32,
    Kwanyama = 0x6B6A0000 as u32,
    Lao = 0x6C6F0000 as u32,
    Latin = 0x6C610000 as u32,
    Latvian__Lettish_ = 0x6C760000 as u32,
    Limburgish___Limburger_ = 0x6C690000 as u32,
    Lingala = 0x6C6E0000 as u32,
    Lithuanian = 0x6C740000 as u32,
    Luga_Katanga = 0x6C750000 as u32,
    Luganda__Ganda = 0x6C670000 as u32,
    Luxembourgish = 0x6C620000 as u32,
    Manx = 0x67760000 as u32,
    Macedonian = 0x6D6B0000 as u32,
    Malagasy = 0x6D670000 as u32,
    Malay = 0x6D730000 as u32,
    Malayalam = 0x6D6C0000 as u32,
    Maltese = 0x6D740000 as u32,
    Maori = 0x6D690000 as u32,
    Marathi = 0x6D720000 as u32,
    Marshallese = 0x6D680000 as u32,
    Moldavian = 0x6D6F0000 as u32,
    Mongolian = 0x6D6E0000 as u32,
    Nauru = 0x6E610000 as u32,
    Navajo = 0x6E760000 as u32,
    Ndonga = 0x6E670000 as u32,
    Northern_Ndebele = 0x6E640000 as u32,
    Nepali = 0x6E650000 as u32,
    Norwegian = 0x6E6F0000 as u32,
    Norwegian_bokmål = 0x6E620000 as u32,
    Norwegian_nynorsk = 0x6E6E0000 as u32,
    Nuosu = 0x69690000 as u32,
    Occitan = 0x6F630000 as u32,
    Ojibwe = 0x6F6A0000 as u32,
    Old_Church_Slavonic__Old_Bulgarian = 0x63750000 as u32,
    Oriya = 0x6F720000 as u32,
    Oromo__Afaan_Oromo_ = 0x6F6D0000 as u32,
    Ossetian = 0x6F730000 as u32,
    Pāli = 0x70690000 as u32,
    Pashto__Pushto = 0x70730000 as u32,
    Persian__Farsi_ = 0x66610000 as u32,
    Polish = 0x706C0000 as u32,
    Portuguese = 0x70740000 as u32,
    Punjabi__Eastern_ = 0x70610000 as u32,
    Quechua = 0x71750000 as u32,
    Romansh = 0x726D0000 as u32,
    Romanian = 0x726F0000 as u32,
    Russian = 0x72750000 as u32,
    Sami = 0x73650000 as u32,
    Samoan = 0x736D0000 as u32,
    Sango = 0x73670000 as u32,
    Sanskrit = 0x73610000 as u32,
    Serbian = 0x73720000 as u32,
    Serbo_Croatian = 0x73680000 as u32,
    Sesotho = 0x73740000 as u32,
    Setswana = 0x746E0000 as u32,
    Shona = 0x736E0000 as u32,
    Sichuan_Yi = 0x69690000 as u32,
    Sindhi = 0x73640000 as u32,
    Sinhalese = 0x73690000 as u32,
    Siswati = 0x73730000 as u32,
    Slovak = 0x736B0000 as u32,
    Slovenian = 0x736C0000 as u32,
    Somali = 0x736F0000 as u32,
    Southern_Ndebele = 0x6E720000 as u32,
    Spanish = 0x65730000 as u32,
    Sundanese = 0x73750000 as u32,
    Swahili__Kiswahili_ = 0x73770000 as u32,
    Swati = 0x73730000 as u32,
    Swedish = 0x73760000 as u32,
    Tagalog = 0x746C0000 as u32,
    Tahitian = 0x74790000 as u32,
    Tajik = 0x74670000 as u32,
    Tamil = 0x74610000 as u32,
    Tatar = 0x74740000 as u32,
    Telugu = 0x74650000 as u32,
    Thai = 0x74680000 as u32,
    Tibetan = 0x626F0000 as u32,
    Tigrinya = 0x74690000 as u32,
    Tonga = 0x746F0000 as u32,
    Tsonga = 0x74730000 as u32,
    Turkish = 0x74720000 as u32,
    Turkmen = 0x746B0000 as u32,
    Twi = 0x74770000 as u32,
    Uyghur = 0x75670000 as u32,
    Ukrainian = 0x756B0000 as u32,
    Urdu = 0x75720000 as u32,
    Uzbek = 0x757A0000 as u32,
    Venda = 0x76650000 as u32,
    Vietnamese = 0x76690000 as u32,
    Volapük = 0x766F0000 as u32,
    Wallon = 0x77610000 as u32,
    Welsh = 0x63790000 as u32,
    Wolof = 0x776F0000 as u32,
    Western_Frisian = 0x66790000 as u32,
    Xhosa = 0x78680000 as u32,
    Yiddish = 0x79690000 as u32,
    Yoruba = 0x796F0000 as u32,
    Zhuang__Chuang = 0x7A610000 as u32,
    Zulu = 0x7A750000 as u32,
}

//define Country Data
enum Country {
    Afghanistan = 0x61660000 as u32,
    Aland_Islands = 0x61780000 as u32,
    Albania = 0x616C0000 as u32,
    Algeria = 0x647A0000 as u32,
    American_Samoa = 0x61730000 as u32,
    Andorra = 0x61640000 as u32,
    Angola = 0x616F0000 as u32,
    Anguilla = 0x61690000 as u32,
    Antarctica = 0x61710000 as u32,
    Antigua_and_Barbuda = 0x61670000 as u32,
    Argentina = 0x61720000 as u32,
    Armenia = 0x616D0000 as u32,
    Aruba = 0x61770000 as u32,
    Australia = 0x61750000 as u32,
    Austria = 0x61740000 as u32,
    Azerbaijan = 0x617A0000 as u32,
    Bahamas = 0x62730000 as u32,
    Bahrain = 0x62680000 as u32,
    Bangladesh = 0x62640000 as u32,
    Barbados = 0x62620000 as u32,
    Belarus = 0x62790000 as u32,
    Belgium = 0x62650000 as u32,
    Belize = 0x627A0000 as u32,
    Benin = 0x626A0000 as u32,
    Bermuda = 0x626D0000 as u32,
    Bhutan = 0x62740000 as u32,
    Bolivia = 0x626F0000 as u32,
    Bonaire__Sint_Eustatius_and_Saba = 0x62710000 as u32,
    Bosnia_and_Herzegovina = 0x62610000 as u32,
    Botswana = 0x62770000 as u32,
    Bouvet_Island = 0x62760000 as u32,
    Brazil = 0x62720000 as u32,
    British_Virgin_Islands = 0x76670000 as u32,
    British_Indian_Ocean_Territory = 0x696F0000 as u32,
    Brunei_Darussalam = 0x626E0000 as u32,
    Bulgaria = 0x62670000 as u32,
    Burkina_Faso = 0x62660000 as u32,
    Burundi = 0x62690000 as u32,
    Cambodia = 0x6B680000 as u32,
    Cameroon = 0x636D0000 as u32,
    Canada = 0x63610000 as u32,
    Cape_Verde = 0x63760000 as u32,
    Cayman_Islands = 0x6B790000 as u32,
    Central_African_Republic = 0x63660000 as u32,
    Chad = 0x74640000 as u32,
    Chile = 0x636C0000 as u32,
    China = 0x636E0000 as u32,
    Hong_Kong__Special_Administrative_Region_of_China = 0x686B0000 as u32,
    Macao__Special_Administrative_Region_of_China = 0x6D6F0000 as u32,
    Christmas_Island = 0x63780000 as u32,
    Cocos__Keeling__Islands = 0x63630000 as u32,
    Colombia = 0x636F0000 as u32,
    Comoros = 0x6B6D0000 as u32,
    Congo__Brazzaville_ = 0x63670000 as u32,
    Congo__Democratic_Republic_of_the = 0x63640000 as u32,
    Cook_Islands = 0x636B0000 as u32,
    Costa_Rica = 0x63720000 as u32,
    Côte_d'Ivoire = 0x63690000 as u32,
    Croatia = 0x68720000 as u32,
    Cuba = 0x63750000 as u32,
    Curaçao = 0x63770000 as u32,
    Cyprus = 0x63790000 as u32,
    Czech_Republic = 0x637A0000 as u32,
    Denmark = 0x646B0000 as u32,
    Djibouti = 0x646A0000 as u32,
    Dominica = 0x646D0000 as u32,
    Dominican_Republic = 0x646F0000 as u32,
    Ecuador = 0x65630000 as u32,
    Egypt = 0x65670000 as u32,
    El_Salvador = 0x73760000 as u32,
    Equatorial_Guinea = 0x67710000 as u32,
    Eritrea = 0x65720000 as u32,
    Estonia = 0x65650000 as u32,
    Ethiopia = 0x65740000 as u32,
    Falkland_Islands__Malvinas_ = 0x666B0000 as u32,
    Faroe_Islands = 0x666F0000 as u32,
    Fiji = 0x666A0000 as u32,
    Finland = 0x66690000 as u32,
    France = 0x66720000 as u32,
    French_Guiana = 0x67660000 as u32,
    French_Polynesia = 0x70660000 as u32,
    French_Southern_Territories = 0x74660000 as u32,
    Gabon = 0x67610000 as u32,
    Gambia = 0x676D0000 as u32,
    Georgia = 0x67650000 as u32,
    Germany = 0x64650000 as u32,
    Ghana = 0x67680000 as u32,
    Gibraltar = 0x67690000 as u32,
    Greece = 0x67720000 as u32,
    Greenland = 0x676C0000 as u32,
    Grenada = 0x67640000 as u32,
    Guadeloupe = 0x67700000 as u32,
    Guam = 0x67750000 as u32,
    Guatemala = 0x67740000 as u32,
    Guernsey = 0x67670000 as u32,
    Guinea = 0x676E0000 as u32,
    Guinea_Bissau = 0x67770000 as u32,
    Guyana = 0x67790000 as u32,
    Haiti = 0x68740000 as u32,
    Heard_Island_and_Mcdonald_Islands = 0x686D0000 as u32,
    Holy_See__Vatican_City_State_ = 0x76610000 as u32,
    Honduras = 0x686E0000 as u32,
    Hungary = 0x68750000 as u32,
    Iceland = 0x69730000 as u32,
    India = 0x696E0000 as u32,
    Indonesia = 0x69640000 as u32,
    Iran__Islamic_Republic_of = 0x69720000 as u32,
    Iraq = 0x69710000 as u32,
    Ireland = 0x69650000 as u32,
    Isle_of_Man = 0x696D0000 as u32,
    Israel = 0x696C0000 as u32,
    Italy = 0x69740000 as u32,
    Jamaica = 0x6A6D0000 as u32,
    Japan = 0x6A700000 as u32,
    Jersey = 0x6A650000 as u32,
    Jordan = 0x6A6F0000 as u32,
    Kazakhstan = 0x6B7A0000 as u32,
    Kenya = 0x6B650000 as u32,
    Kiribati = 0x6B690000 as u32,
    Korea__Democratic_People's_Republic_of = 0x6B700000 as u32,
    Korea__Republic_of = 0x6B720000 as u32,
    Kuwait = 0x6B770000 as u32,
    Kyrgyzstan = 0x6B670000 as u32,
    Lao_PDR = 0x6C610000 as u32,
    Latvia = 0x6C760000 as u32,
    Lebanon = 0x6C620000 as u32,
    Lesotho = 0x6C730000 as u32,
    Liberia = 0x6C720000 as u32,
    Libya = 0x6C790000 as u32,
    Liechtenstein = 0x6C690000 as u32,
    Lithuania = 0x6C740000 as u32,
    Luxembourg = 0x6C750000 as u32,
    Macedonia__Republic_of = 0x6D6B0000 as u32,
    Madagascar = 0x6D670000 as u32,
    Malawi = 0x6D770000 as u32,
    Malaysia = 0x6D790000 as u32,
    Maldives = 0x6D760000 as u32,
    Mali = 0x6D6C0000 as u32,
    Malta = 0x6D740000 as u32,
    Marshall_Islands = 0x6D680000 as u32,
    Martinique = 0x6D710000 as u32,
    Mauritania = 0x6D720000 as u32,
    Mauritius = 0x6D750000 as u32,
    Mayotte = 0x79740000 as u32,
    Mexico = 0x6D780000 as u32,
    Micronesia__Federated_States_of = 0x666D0000 as u32,
    Moldova = 0x6D640000 as u32,
    Monaco = 0x6D630000 as u32,
    Mongolia = 0x6D6E0000 as u32,
    Montenegro = 0x6D650000 as u32,
    Montserrat = 0x6D730000 as u32,
    Morocco = 0x6D610000 as u32,
    Mozambique = 0x6D7A0000 as u32,
    Myanmar = 0x6D6D0000 as u32,
    Namibia = 0x6E610000 as u32,
    Nauru = 0x6E720000 as u32,
    Nepal = 0x6E700000 as u32,
    Netherlands = 0x6E6C0000 as u32,
    Netherlands_Antilles = 0x616E0000 as u32,
    New_Caledonia = 0x6E630000 as u32,
    New_Zealand = 0x6E7A0000 as u32,
    Nicaragua = 0x6E690000 as u32,
    Niger = 0x6E650000 as u32,
    Nigeria = 0x6E670000 as u32,
    Niue = 0x6E750000 as u32,
    Norfolk_Island = 0x6E660000 as u32,
    Northern_Mariana_Islands = 0x6D700000 as u32,
    Norway = 0x6E6F0000 as u32,
    Oman = 0x6F6D0000 as u32,
    Pakistan = 0x706B0000 as u32,
    Palau = 0x70770000 as u32,
    Palestinian_Territory__Occupied = 0x70730000 as u32,
    Panama = 0x70610000 as u32,
    Papua_New_Guinea = 0x70670000 as u32,
    Paraguay = 0x70790000 as u32,
    Peru = 0x70650000 as u32,
    Philippines = 0x70680000 as u32,
    Pitcairn = 0x706E0000 as u32,
    Poland = 0x706C0000 as u32,
    Portugal = 0x70740000 as u32,
    Puerto_Rico = 0x70720000 as u32,
    Qatar = 0x71610000 as u32,
    Réunion = 0x72650000 as u32,
    Romania = 0x726F0000 as u32,
    Russian_Federation = 0x72750000 as u32,
    Rwanda = 0x72770000 as u32,
    Saint_Barthélemy = 0x626C0000 as u32,
    Saint_Helena = 0x73680000 as u32,
    Saint_Kitts_and_Nevis = 0x6B6E0000 as u32,
    Saint_Lucia = 0x6C630000 as u32,
    Saint_Martin__French_part_ = 0x6D660000 as u32,
    Saint_Pierre_and_Miquelon = 0x706D0000 as u32,
    Saint_Vincent_and_Grenadines = 0x76630000 as u32,
    Samoa = 0x77730000 as u32,
    San_Marino = 0x736D0000 as u32,
    Sao_Tome_and_Principe = 0x73740000 as u32,
    Saudi_Arabia = 0x73610000 as u32,
    Senegal = 0x736E0000 as u32,
    Serbia = 0x72730000 as u32,
    Seychelles = 0x73630000 as u32,
    Sierra_Leone = 0x736C0000 as u32,
    Singapore = 0x73670000 as u32,
    Sint_Maarten__Dutch_part_ = 0x73780000 as u32,
    Slovakia = 0x736B0000 as u32,
    Slovenia = 0x73690000 as u32,
    Solomon_Islands = 0x73620000 as u32,
    Somalia = 0x736F0000 as u32,
    South_Africa = 0x7A610000 as u32,
    South_Georgia_and_the_South_Sandwich_Islands = 0x67730000 as u32,
    South_Sudan = 0x73730000 as u32,
    Spain = 0x65730000 as u32,
    Sri_Lanka = 0x6C6B0000 as u32,
    Sudan = 0x73640000 as u32,
    Suriname_* = 0x73720000 as u32,
    Svalbard_and_Jan_Mayen_Islands = 0x736A0000 as u32,
    Swaziland = 0x737A0000 as u32,
    Sweden = 0x73650000 as u32,
    Switzerland = 0x63680000 as u32,
    Syrian_Arab_Republic__Syria_ = 0x73790000 as u32,
    Taiwan = 0x74770000 as u32,
    Tajikistan = 0x746A0000 as u32,
    Tanzania_*__United_Republic_of = 0x747A0000 as u32,
    Thailand = 0x74680000 as u32,
    Timor_Leste = 0x746C0000 as u32,
    Togo = 0x74670000 as u32,
    Tokelau = 0x746B0000 as u32,
    Tonga = 0x746F0000 as u32,
    Trinidad_and_Tobago = 0x74740000 as u32,
    Tunisia = 0x746E0000 as u32,
    Turkey = 0x74720000 as u32,
    Turkmenistan = 0x746D0000 as u32,
    Turks_and_Caicos_Islands = 0x74630000 as u32,
    Tuvalu = 0x74760000 as u32,
    Uganda = 0x75670000 as u32,
    Ukraine = 0x75610000 as u32,
    United_Arab_Emirates = 0x61650000 as u32,
    United_Kingdom = 0x67620000 as u32,
    United_States_of_America = 0x75730000 as u32,
    United_States_Minor_Outlying_Islands = 0x756D0000 as u32,
    Uruguay = 0x75790000 as u32,
    Uzbekistan = 0x757A0000 as u32,
    Vanuatu = 0x76750000 as u32,
    Venezuela__Bolivarian_Republic_of_ = 0x76650000 as u32,
    Viet_Nam = 0x766E0000 as u32,
    Virgin_Islands__US = 0x76690000 as u32,
    Wallis_and_Futuna_Islands = 0x77660000 as u32,
    Western_Sahara = 0x65680000 as u32,
    Yemen = 0x79650000 as u32,
    Zambia = 0x7A6D0000 as u32,
    Zimbabwe = 0x7A770000 as u32,
}

enum CharSet {
    charset_unspec = 0u16,
    charset_iso_10646_ucs_2 = 1000u16,
    charset_iso_10646_ucs_4 = 1001u16,
    charset_iso_8859_1 = 4u16,
    charset_iso_8859_2 = 5u16,
    charset_iso_8859_3 = 6u16,
    charset_iso_8859_4 = 7u16,
    charset_iso_8859_5 = 8u16,
    charset_iso_8859_6 = 9u16,
    charset_iso_8859_7 = 10u16,
    charset_iso_8859_8 = 11u16,
    charset_iso_8859_9 = 12u16,
    charset_iso_8859_10 = 13u16,
    charset_iso_8859_13 = 109u16,
    charset_iso_8859_14 = 110u16,
    charset_iso_8859_15 = 111u16,
    charset_es_c_5601 = 36u16,
    charset_iso_2022_kr = 37u16,
    charset_iso_2022_jp = 39u16,
    charset_iso_2022_jp_2 = 40u16,
    charset_jis_x_0208 = 63u16,
    charset_iso_2022_cn = 104u16,
    charset_gb_2312 = 2025u16,
}

enum StringFlags {
    str_flag_nt = "0", //strings shall be null terminated
}

struct StringSpec {
    str_max_len: u16,
    str_flags: StringFlags,
}

struct Locale {
    language: Language,
    country: Country,
    charset: CharSet,
    str_spec: StringSpec,
}

struct ExtNomenRef {
    nomenclature_id: OidType,
    nomenclature_code: any,
}

struct ExtObjRelationEntry {
    relation_type: OidType,
    related_object: OidType,
    relation_attributes: AttributeList
}

type ExtObjRelationList = Array<ExtObjRelationEntry>;


