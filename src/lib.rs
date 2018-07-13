#![allow(non_snake_case, non_camel_case_types, dead_code)]

extern crate libc;

pub type   HPDF_INT = libc::c_int;
pub type   HPDF_UINT = libc::c_uint;


/*  32bit integer types
 */
pub type   HPDF_INT32 = libc::int32_t;
pub type   HPDF_UINT32 = libc::uint32_t;


/*  16bit integer types
 */
pub type   HPDF_INT16 = libc::int16_t;
pub type   HPDF_UINT16 = libc::uint16_t;


/*  8bit integer types
 */
pub type   HPDF_INT8 = libc::c_schar;
pub type   HPDF_UINT8 = libc::c_uchar;


/*  8bit binary types
 */
pub type   HPDF_BYTE = libc::c_uchar;


/*  float type (32bit IEEE754)
 */
pub type   HPDF_REAL = libc::c_float;


/*  double type (64bit IEEE754)
 */
pub type   HPDF_DOUBLE = libc::c_double;


/*  boolean type (0: False, !0: True)
 */
pub type   HPDF_BOOL = libc::c_int;


/*  error-no type (32bit unsigned integer)
 */
pub type   HPDF_STATUS = libc::uint32_t;


/*  charactor-code type (16bit)
 */
type  HPDF_CID      = HPDF_UINT16;
type  HPDF_UNICODE  = HPDF_UINT16;

/*  HPDF_Point struct
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct  HPDF_Point {
    pub x: HPDF_REAL,
    pub y: HPDF_REAL,
} 

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HPDF_Rect {
    pub left: HPDF_REAL,
    pub bottom: HPDF_REAL,
    pub right: HPDF_REAL,
    pub top: HPDF_REAL,
} 

/*  HPDF_Point3D struct
*/
#[repr(C)]
struct  HPDF_Point3D {
	x: HPDF_REAL,
	y: HPDF_REAL,
	z: HPDF_REAL,
}

type HPDF_Box = HPDF_Rect;

/* HPDF_Date struct
*/ 
#[repr(C)]
struct  HPDF_Date {
   year:        HPDF_INT,
   month:       HPDF_INT,
   day:         HPDF_INT,
   hour:        HPDF_INT,
   minutes:     HPDF_INT,
   seconds:     HPDF_INT,
   ind:         libc::c_char,
   off_hour:    HPDF_INT,
   off_minutes: HPDF_INT,
} 

#[repr(C)]
pub enum HPDF_InfoType {
    /* date-time type parameters */
    HPDF_INFO_CREATION_DATE = 0,
    HPDF_INFO_MOD_DATE,

    /* string type parameters */
    HPDF_INFO_AUTHOR,
    HPDF_INFO_CREATOR,
    HPDF_INFO_PRODUCER,
    HPDF_INFO_TITLE,
    HPDF_INFO_SUBJECT,
    HPDF_INFO_KEYWORDS,
    HPDF_INFO_TRAPPED,
    HPDF_INFO_GTS_PDFX,
    HPDF_INFO_EOF
}

/* PDF-A Types */

#[repr(C)]
pub enum HPDF_PDFA_TYPE
{
    HPDF_PDFA_1A = 0,
    HPDF_PDFA_1B = 1
} 


#[repr(C)]
pub enum _HPDF_PdfVer {
    HPDF_VER_12 = 0,
    HPDF_VER_13,
    HPDF_VER_14,
    HPDF_VER_15,
    HPDF_VER_16,
    HPDF_VER_17,
    HPDF_VER_EOF
} 

#[repr(C)]
pub enum  HPDF_EncryptMode {
    HPDF_ENCRYPT_R2    = 2,
    HPDF_ENCRYPT_R3    = 3
}


//Constants

/*----------------------------------------------------------------------------*/

pub const  HPDF_TRUE      :  i32 =          1;
pub const  HPDF_FALSE     :  i32 =          0;
 
pub const  HPDF_OK        :  u8 =          0;
pub const  HPDF_NOERROR   :  u8 =          0;

/*----- default values -------------------------------------------------------*/

/* buffer size which is required when we convert to character string. */
pub const HPDF_TMP_BUF_SIZ       : u16 =     512;
pub const HPDF_SHORT_BUF_SIZ     : u16 =     32;
pub const HPDF_REAL_LEN          : u16 =     11;
pub const HPDF_INT_LEN           : u16 =     11;
pub const HPDF_TEXT_DEFAULT_LEN  : u16 =     256;
pub const HPDF_UNICODE_HEADER_LEN: u16 =     2;
pub const HPDF_DATE_TIME_STR_LEN : u16 =     23;

/* length of each item defined in PDF */
pub const HPDF_BYTE_OFFSET_LEN : u8 =       10;
pub const HPDF_OBJ_ID_LEN      : u8 =       7;
pub const HPDF_GEN_NO_LEN      : u8 =       5;

/* default value of Graphic State */
//#define HPDF_DEF_FONT               "Helvetica"
//#define HPDF_DEF_PAGE_LAYOUT        HPDF_PAGE_LAYOUT_SINGLE
//#define HPDF_DEF_PAGE_MODE          HPDF_PAGE_MODE_USE_NONE
//#define HPDF_DEF_WORDSPACE          0
//#define HPDF_DEF_CHARSPACE          0
//#define HPDF_DEF_FONTSIZE           10
//#define HPDF_DEF_HSCALING           100
//#define HPDF_DEF_LEADING            0
//#define HPDF_DEF_RENDERING_MODE     HPDF_FILL
//#define HPDF_DEF_RISE               0
//#define HPDF_DEF_RAISE              HPDF_DEF_RISE
//#define HPDF_DEF_LINEWIDTH          1
//#define HPDF_DEF_LINECAP            HPDF_BUTT_END
//#define HPDF_DEF_LINEJOIN           HPDF_MITER_JOIN
pub const HPDF_DEF_MITERLIMIT:u8      =   10;
//#define HPDF_DEF_FLATNESS           1
//#define HPDF_DEF_PAGE_NUM           1
//
//#define HPDF_BS_DEF_WIDTH           1
//
///* defalt page-size */
//#define HPDF_DEF_PAGE_WIDTH         595.276F
//#define HPDF_DEF_PAGE_HEIGHT        841.89F
//
///*---------------------------------------------------------------------------*/
///*----- compression mode ----------------------------------------------------*/
//
pub const  HPDF_COMP_NONE     : u32 =       0x00;
pub const  HPDF_COMP_TEXT     : u32 =       0x01;
pub const  HPDF_COMP_IMAGE    : u32 =       0x02;
pub const  HPDF_COMP_METADATA : u32 =       0x04;
pub const  HPDF_COMP_ALL      : u32 =       0x0F;
///* #define  HPDF_COMP_BEST_COMPRESS   0x10
// * #define  HPDF_COMP_BEST_SPEED      0x20
// */
pub const  HPDF_COMP_MASK     : u32 =       0xFF;
//
//
///*----------------------------------------------------------------------------*/
///*----- permission flags (only Revision 2 is supported)-----------------------*/
//
pub const HPDF_ENABLE_READ      : u32 =  0;
pub const HPDF_ENABLE_PRINT     : u32 =  4;
pub const HPDF_ENABLE_EDIT_ALL  : u32 =  8;
pub const HPDF_ENABLE_COPY      : u32 =  16;
pub const HPDF_ENABLE_EDIT      : u32 =  32;
//
//
///*----------------------------------------------------------------------------*/
///*------ viewer preferences definitions --------------------------------------*/
//
//#define HPDF_HIDE_TOOLBAR    1
//#define HPDF_HIDE_MENUBAR    2
//#define HPDF_HIDE_WINDOW_UI  4
//#define HPDF_FIT_WINDOW      8
//#define HPDF_CENTER_WINDOW   16
//#define HPDF_PRINT_SCALING_NONE   32
//
//
///*---------------------------------------------------------------------------*/
///*------ limitation of object implementation (PDF1.4) -----------------------*/
//
//#define HPDF_LIMIT_MAX_INT             2147483647
//#define HPDF_LIMIT_MIN_INT             -2147483647
//
//#define HPDF_LIMIT_MAX_REAL             3.4E38f // per PDF 1.7 spec, Annex C, old value  32767
//#define HPDF_LIMIT_MIN_REAL            -3.4E38f // per PDF 1.7 spec, Annex C, old value -32767
//
//#define HPDF_LIMIT_MAX_STRING_LEN      2147483646 // per PDF 1.7 spec, limit 32767 is for strings in content stream and no limit in other cases => open the limit to max Integer, old value 65535
//#define HPDF_LIMIT_MAX_NAME_LEN        127
//
//#define HPDF_LIMIT_MAX_ARRAY           8388607  // per PDF 1.7 spec, "Maximum number of indirect objects in a PDF file" is 8388607, old value 8191
//#define HPDF_LIMIT_MAX_DICT_ELEMENT    8388607  // per PDF 1.7 spec, "Maximum number of indirect objects in a PDF file" is 8388607, old value 4095
//#define HPDF_LIMIT_MAX_XREF_ELEMENT    8388607
//#define HPDF_LIMIT_MAX_GSTATE          28
//#define HPDF_LIMIT_MAX_DEVICE_N        8
//#define HPDF_LIMIT_MAX_DEVICE_N_V15    32
//#define HPDF_LIMIT_MAX_CID             65535
//#define HPDF_MAX_GENERATION_NUM        65535
//
//#define HPDF_MIN_PAGE_HEIGHT           3
//#define HPDF_MIN_PAGE_WIDTH            3
//#define HPDF_MAX_PAGE_HEIGHT           14400
//#define HPDF_MAX_PAGE_WIDTH            14400
//#define HPDF_MIN_MAGNIFICATION_FACTOR  8
//#define HPDF_MAX_MAGNIFICATION_FACTOR  3200
//
///*---------------------------------------------------------------------------*/
///*------ limitation of various properties -----------------------------------*/
//
//#define HPDF_MIN_PAGE_SIZE          3
//#define HPDF_MAX_PAGE_SIZE          14400
//#define HPDF_MIN_HORIZONTALSCALING  10
//#define HPDF_MAX_HORIZONTALSCALING  300
//#define HPDF_MIN_WORDSPACE          -30
//#define HPDF_MAX_WORDSPACE          300
//#define HPDF_MIN_CHARSPACE          -30
//#define HPDF_MAX_CHARSPACE          300
//#define HPDF_MAX_FONTSIZE           600
//#define HPDF_MAX_ZOOMSIZE           10
//#define HPDF_MAX_LEADING            300
//#define HPDF_MAX_LINEWIDTH          100
//#define HPDF_MAX_DASH_PATTERN       100
//
//#define HPDF_MAX_JWW_NUM            128
//
///*----------------------------------------------------------------------------*/
///*----- country code definition ----------------------------------------------*/
//
//#define HPDF_COUNTRY_AF  "AF"    /* AFGHANISTAN */
//#define HPDF_COUNTRY_AL  "AL"    /* ALBANIA */
//#define HPDF_COUNTRY_DZ  "DZ"    /* ALGERIA */
//#define HPDF_COUNTRY_AS  "AS"    /* AMERICAN SAMOA */
//#define HPDF_COUNTRY_AD  "AD"    /* ANDORRA */
//#define HPDF_COUNTRY_AO  "AO"    /* ANGOLA */
//#define HPDF_COUNTRY_AI  "AI"    /* ANGUILLA */
//#define HPDF_COUNTRY_AQ  "AQ"    /* ANTARCTICA */
//#define HPDF_COUNTRY_AG  "AG"    /* ANTIGUA AND BARBUDA */
//#define HPDF_COUNTRY_AR  "AR"    /* ARGENTINA */
//#define HPDF_COUNTRY_AM  "AM"    /* ARMENIA */
//#define HPDF_COUNTRY_AW  "AW"    /* ARUBA */
//#define HPDF_COUNTRY_AU  "AU"    /* AUSTRALIA */
//#define HPDF_COUNTRY_AT  "AT"    /* AUSTRIA */
//#define HPDF_COUNTRY_AZ  "AZ"    /* AZERBAIJAN */
//#define HPDF_COUNTRY_BS  "BS"    /* BAHAMAS */
//#define HPDF_COUNTRY_BH  "BH"    /* BAHRAIN */
//#define HPDF_COUNTRY_BD  "BD"    /* BANGLADESH */
//#define HPDF_COUNTRY_BB  "BB"    /* BARBADOS */
//#define HPDF_COUNTRY_BY  "BY"    /* BELARUS */
//#define HPDF_COUNTRY_BE  "BE"    /* BELGIUM */
//#define HPDF_COUNTRY_BZ  "BZ"    /* BELIZE */
//#define HPDF_COUNTRY_BJ  "BJ"    /* BENIN */
//#define HPDF_COUNTRY_BM  "BM"    /* BERMUDA */
//#define HPDF_COUNTRY_BT  "BT"    /* BHUTAN */
//#define HPDF_COUNTRY_BO  "BO"    /* BOLIVIA */
//#define HPDF_COUNTRY_BA  "BA"    /* BOSNIA AND HERZEGOWINA */
//#define HPDF_COUNTRY_BW  "BW"    /* BOTSWANA */
//#define HPDF_COUNTRY_BV  "BV"    /* BOUVET ISLAND */
//#define HPDF_COUNTRY_BR  "BR"    /* BRAZIL */
//#define HPDF_COUNTRY_IO  "IO"    /* BRITISH INDIAN OCEAN TERRITORY */
//#define HPDF_COUNTRY_BN  "BN"    /* BRUNEI DARUSSALAM */
//#define HPDF_COUNTRY_BG  "BG"    /* BULGARIA */
//#define HPDF_COUNTRY_BF  "BF"    /* BURKINA FASO */
//#define HPDF_COUNTRY_BI  "BI"    /* BURUNDI */
//#define HPDF_COUNTRY_KH  "KH"    /* CAMBODIA */
//#define HPDF_COUNTRY_CM  "CM"    /* CAMEROON */
//#define HPDF_COUNTRY_CA  "CA"    /* CANADA */
//#define HPDF_COUNTRY_CV  "CV"    /* CAPE VERDE */
//#define HPDF_COUNTRY_KY  "KY"    /* CAYMAN ISLANDS */
//#define HPDF_COUNTRY_CF  "CF"    /* CENTRAL AFRICAN REPUBLIC */
//#define HPDF_COUNTRY_TD  "TD"    /* CHAD */
//#define HPDF_COUNTRY_CL  "CL"    /* CHILE */
//#define HPDF_COUNTRY_CN  "CN"    /* CHINA */
//#define HPDF_COUNTRY_CX  "CX"    /* CHRISTMAS ISLAND */
//#define HPDF_COUNTRY_CC  "CC"    /* COCOS (KEELING) ISLANDS */
//#define HPDF_COUNTRY_CO  "CO"    /* COLOMBIA */
//#define HPDF_COUNTRY_KM  "KM"    /* COMOROS */
//#define HPDF_COUNTRY_CG  "CG"    /* CONGO */
//#define HPDF_COUNTRY_CK  "CK"    /* COOK ISLANDS */
//#define HPDF_COUNTRY_CR  "CR"    /* COSTA RICA */
//#define HPDF_COUNTRY_CI  "CI"    /* COTE D'IVOIRE */
//#define HPDF_COUNTRY_HR  "HR"    /* CROATIA (local name: Hrvatska) */
//#define HPDF_COUNTRY_CU  "CU"    /* CUBA */
//#define HPDF_COUNTRY_CY  "CY"    /* CYPRUS */
//#define HPDF_COUNTRY_CZ  "CZ"    /* CZECH REPUBLIC */
//#define HPDF_COUNTRY_DK  "DK"    /* DENMARK */
//#define HPDF_COUNTRY_DJ  "DJ"    /* DJIBOUTI */
//#define HPDF_COUNTRY_DM  "DM"    /* DOMINICA */
//#define HPDF_COUNTRY_DO  "DO"    /* DOMINICAN REPUBLIC */
//#define HPDF_COUNTRY_TP  "TP"    /* EAST TIMOR */
//#define HPDF_COUNTRY_EC  "EC"    /* ECUADOR */
//#define HPDF_COUNTRY_EG  "EG"    /* EGYPT */
//#define HPDF_COUNTRY_SV  "SV"    /* EL SALVADOR */
//#define HPDF_COUNTRY_GQ  "GQ"    /* EQUATORIAL GUINEA */
//#define HPDF_COUNTRY_ER  "ER"    /* ERITREA */
//#define HPDF_COUNTRY_EE  "EE"    /* ESTONIA */
//#define HPDF_COUNTRY_ET  "ET"    /* ETHIOPIA */
//#define HPDF_COUNTRY_FK  "FK"   /* FALKLAND ISLANDS (MALVINAS) */
//#define HPDF_COUNTRY_FO  "FO"    /* FAROE ISLANDS */
//#define HPDF_COUNTRY_FJ  "FJ"    /* FIJI */
//#define HPDF_COUNTRY_FI  "FI"    /* FINLAND */
//#define HPDF_COUNTRY_FR  "FR"    /* FRANCE */
//#define HPDF_COUNTRY_FX  "FX"    /* FRANCE, METROPOLITAN */
//#define HPDF_COUNTRY_GF  "GF"    /* FRENCH GUIANA */
//#define HPDF_COUNTRY_PF  "PF"    /* FRENCH POLYNESIA */
//#define HPDF_COUNTRY_TF  "TF"    /* FRENCH SOUTHERN TERRITORIES */
//#define HPDF_COUNTRY_GA  "GA"    /* GABON */
//#define HPDF_COUNTRY_GM  "GM"    /* GAMBIA */
//#define HPDF_COUNTRY_GE  "GE"    /* GEORGIA */
//#define HPDF_COUNTRY_DE  "DE"    /* GERMANY */
//#define HPDF_COUNTRY_GH  "GH"    /* GHANA */
//#define HPDF_COUNTRY_GI  "GI"    /* GIBRALTAR */
//#define HPDF_COUNTRY_GR  "GR"    /* GREECE */
//#define HPDF_COUNTRY_GL  "GL"    /* GREENLAND */
//#define HPDF_COUNTRY_GD  "GD"    /* GRENADA */
//#define HPDF_COUNTRY_GP  "GP"    /* GUADELOUPE */
//#define HPDF_COUNTRY_GU  "GU"    /* GUAM */
//#define HPDF_COUNTRY_GT  "GT"    /* GUATEMALA */
//#define HPDF_COUNTRY_GN  "GN"    /* GUINEA */
//#define HPDF_COUNTRY_GW  "GW"    /* GUINEA-BISSAU */
//#define HPDF_COUNTRY_GY  "GY"    /* GUYANA */
//#define HPDF_COUNTRY_HT  "HT"    /* HAITI */
//#define HPDF_COUNTRY_HM  "HM"    /* HEARD AND MC DONALD ISLANDS */
//#define HPDF_COUNTRY_HN  "HN"    /* HONDURAS */
//#define HPDF_COUNTRY_HK  "HK"    /* HONG KONG */
//#define HPDF_COUNTRY_HU  "HU"    /* HUNGARY */
//#define HPDF_COUNTRY_IS  "IS"    /* ICELAND */
//#define HPDF_COUNTRY_IN  "IN"    /* INDIA */
//#define HPDF_COUNTRY_ID  "ID"    /* INDONESIA */
//#define HPDF_COUNTRY_IR  "IR"    /* IRAN (ISLAMIC REPUBLIC OF) */
//#define HPDF_COUNTRY_IQ  "IQ"    /* IRAQ */
//#define HPDF_COUNTRY_IE  "IE"    /* IRELAND */
//#define HPDF_COUNTRY_IL  "IL"    /* ISRAEL */
//#define HPDF_COUNTRY_IT  "IT"    /* ITALY */
//#define HPDF_COUNTRY_JM  "JM"    /* JAMAICA */
//#define HPDF_COUNTRY_JP  "JP"    /* JAPAN */
//#define HPDF_COUNTRY_JO  "JO"    /* JORDAN */
//#define HPDF_COUNTRY_KZ  "KZ"    /* KAZAKHSTAN */
//#define HPDF_COUNTRY_KE  "KE"    /* KENYA */
//#define HPDF_COUNTRY_KI  "KI"    /* KIRIBATI */
//#define HPDF_COUNTRY_KP  "KP"    /* KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF */
//#define HPDF_COUNTRY_KR  "KR"    /* KOREA, REPUBLIC OF */
//#define HPDF_COUNTRY_KW  "KW"    /* KUWAIT */
//#define HPDF_COUNTRY_KG  "KG"    /* KYRGYZSTAN */
//#define HPDF_COUNTRY_LA  "LA"    /* LAO PEOPLE'S DEMOCRATIC REPUBLIC */
//#define HPDF_COUNTRY_LV  "LV"    /* LATVIA */
//#define HPDF_COUNTRY_LB  "LB"    /* LEBANON */
//#define HPDF_COUNTRY_LS  "LS"    /* LESOTHO */
//#define HPDF_COUNTRY_LR  "LR"    /* LIBERIA */
//#define HPDF_COUNTRY_LY  "LY"    /* LIBYAN ARAB JAMAHIRIYA */
//#define HPDF_COUNTRY_LI  "LI"    /* LIECHTENSTEIN */
//#define HPDF_COUNTRY_LT  "LT"    /* LITHUANIA */
//#define HPDF_COUNTRY_LU  "LU"    /* LUXEMBOURG */
//#define HPDF_COUNTRY_MO  "MO"    /* MACAU */
//#define HPDF_COUNTRY_MK  "MK"   /* MACEDONIA, THE FORMER YUGOSLAV REPUBLIC OF */
//#define HPDF_COUNTRY_MG  "MG"    /* MADAGASCAR */
//#define HPDF_COUNTRY_MW  "MW"    /* MALAWI */
//#define HPDF_COUNTRY_MY  "MY"    /* MALAYSIA */
//#define HPDF_COUNTRY_MV  "MV"    /* MALDIVES */
//#define HPDF_COUNTRY_ML  "ML"    /* MALI */
//#define HPDF_COUNTRY_MT  "MT"    /* MALTA */
//#define HPDF_COUNTRY_MH  "MH"    /* MARSHALL ISLANDS */
//#define HPDF_COUNTRY_MQ  "MQ"    /* MARTINIQUE */
//#define HPDF_COUNTRY_MR  "MR"    /* MAURITANIA */
//#define HPDF_COUNTRY_MU  "MU"    /* MAURITIUS */
//#define HPDF_COUNTRY_YT  "YT"    /* MAYOTTE */
//#define HPDF_COUNTRY_MX  "MX"    /* MEXICO */
//#define HPDF_COUNTRY_FM  "FM"    /* MICRONESIA, FEDERATED STATES OF */
//#define HPDF_COUNTRY_MD  "MD"    /* MOLDOVA, REPUBLIC OF */
//#define HPDF_COUNTRY_MC  "MC"    /* MONACO */
//#define HPDF_COUNTRY_MN  "MN"    /* MONGOLIA */
//#define HPDF_COUNTRY_MS  "MS"    /* MONTSERRAT */
//#define HPDF_COUNTRY_MA  "MA"    /* MOROCCO */
//#define HPDF_COUNTRY_MZ  "MZ"    /* MOZAMBIQUE */
//#define HPDF_COUNTRY_MM  "MM"    /* MYANMAR */
//#define HPDF_COUNTRY_NA  "NA"    /* NAMIBIA */
//#define HPDF_COUNTRY_NR  "NR"    /* NAURU */
//#define HPDF_COUNTRY_NP  "NP"    /* NEPAL */
//#define HPDF_COUNTRY_NL  "NL"    /* NETHERLANDS */
//#define HPDF_COUNTRY_AN  "AN"    /* NETHERLANDS ANTILLES */
//#define HPDF_COUNTRY_NC  "NC"    /* NEW CALEDONIA */
//#define HPDF_COUNTRY_NZ  "NZ"    /* NEW ZEALAND */
//#define HPDF_COUNTRY_NI  "NI"    /* NICARAGUA */
//#define HPDF_COUNTRY_NE  "NE"    /* NIGER */
//#define HPDF_COUNTRY_NG  "NG"    /* NIGERIA */
//#define HPDF_COUNTRY_NU  "NU"    /* NIUE */
//#define HPDF_COUNTRY_NF  "NF"    /* NORFOLK ISLAND */
//#define HPDF_COUNTRY_MP  "MP"    /* NORTHERN MARIANA ISLANDS */
//#define HPDF_COUNTRY_NO  "NO"    /* NORWAY */
//#define HPDF_COUNTRY_OM  "OM"    /* OMAN */
//#define HPDF_COUNTRY_PK  "PK"    /* PAKISTAN */
//#define HPDF_COUNTRY_PW  "PW"    /* PALAU */
//#define HPDF_COUNTRY_PA  "PA"    /* PANAMA */
//#define HPDF_COUNTRY_PG  "PG"    /* PAPUA NEW GUINEA */
//#define HPDF_COUNTRY_PY  "PY"    /* PARAGUAY */
//#define HPDF_COUNTRY_PE  "PE"    /* PERU */
//#define HPDF_COUNTRY_PH  "PH"    /* PHILIPPINES */
//#define HPDF_COUNTRY_PN  "PN"    /* PITCAIRN */
//#define HPDF_COUNTRY_PL  "PL"    /* POLAND */
//#define HPDF_COUNTRY_PT  "PT"    /* PORTUGAL */
//#define HPDF_COUNTRY_PR  "PR"    /* PUERTO RICO */
//#define HPDF_COUNTRY_QA  "QA"    /* QATAR */
//#define HPDF_COUNTRY_RE  "RE"    /* REUNION */
//#define HPDF_COUNTRY_RO  "RO"    /* ROMANIA */
//#define HPDF_COUNTRY_RU  "RU"    /* RUSSIAN FEDERATION */
//#define HPDF_COUNTRY_RW  "RW"    /* RWANDA */
//#define HPDF_COUNTRY_KN  "KN"    /* SAINT KITTS AND NEVIS */
//#define HPDF_COUNTRY_LC  "LC"    /* SAINT LUCIA */
//#define HPDF_COUNTRY_VC  "VC"    /* SAINT VINCENT AND THE GRENADINES */
//#define HPDF_COUNTRY_WS  "WS"    /* SAMOA */
//#define HPDF_COUNTRY_SM  "SM"    /* SAN MARINO */
//#define HPDF_COUNTRY_ST  "ST"    /* SAO TOME AND PRINCIPE */
//#define HPDF_COUNTRY_SA  "SA"    /* SAUDI ARABIA */
//#define HPDF_COUNTRY_SN  "SN"    /* SENEGAL */
//#define HPDF_COUNTRY_SC  "SC"    /* SEYCHELLES */
//#define HPDF_COUNTRY_SL  "SL"    /* SIERRA LEONE */
//#define HPDF_COUNTRY_SG  "SG"    /* SINGAPORE */
//#define HPDF_COUNTRY_SK  "SK"    /* SLOVAKIA (Slovak Republic) */
//#define HPDF_COUNTRY_SI  "SI"    /* SLOVENIA */
//#define HPDF_COUNTRY_SB  "SB"    /* SOLOMON ISLANDS */
//#define HPDF_COUNTRY_SO  "SO"    /* SOMALIA */
//#define HPDF_COUNTRY_ZA  "ZA"    /* SOUTH AFRICA */
//#define HPDF_COUNTRY_ES  "ES"    /* SPAIN */
//#define HPDF_COUNTRY_LK  "LK"    /* SRI LANKA */
//#define HPDF_COUNTRY_SH  "SH"    /* ST. HELENA */
//#define HPDF_COUNTRY_PM  "PM"    /* ST. PIERRE AND MIQUELON */
//#define HPDF_COUNTRY_SD  "SD"    /* SUDAN */
//#define HPDF_COUNTRY_SR  "SR"    /* SURINAME */
//#define HPDF_COUNTRY_SJ  "SJ"    /* SVALBARD AND JAN MAYEN ISLANDS */
//#define HPDF_COUNTRY_SZ  "SZ"    /* SWAZILAND */
//#define HPDF_COUNTRY_SE  "SE"    /* SWEDEN */
//#define HPDF_COUNTRY_CH  "CH"    /* SWITZERLAND */
//#define HPDF_COUNTRY_SY  "SY"    /* SYRIAN ARAB REPUBLIC */
//#define HPDF_COUNTRY_TW  "TW"    /* TAIWAN, PROVINCE OF CHINA */
//#define HPDF_COUNTRY_TJ  "TJ"    /* TAJIKISTAN */
//#define HPDF_COUNTRY_TZ  "TZ"    /* TANZANIA, UNITED REPUBLIC OF */
//#define HPDF_COUNTRY_TH  "TH"    /* THAILAND */
//#define HPDF_COUNTRY_TG  "TG"    /* TOGO */
//#define HPDF_COUNTRY_TK  "TK"    /* TOKELAU */
//#define HPDF_COUNTRY_TO  "TO"    /* TONGA */
//#define HPDF_COUNTRY_TT  "TT"    /* TRINIDAD AND TOBAGO */
//#define HPDF_COUNTRY_TN  "TN"    /* TUNISIA */
//#define HPDF_COUNTRY_TR  "TR"    /* TURKEY */
//#define HPDF_COUNTRY_TM  "TM"    /* TURKMENISTAN */
//#define HPDF_COUNTRY_TC  "TC"    /* TURKS AND CAICOS ISLANDS */
//#define HPDF_COUNTRY_TV  "TV"    /* TUVALU */
//#define HPDF_COUNTRY_UG  "UG"    /* UGANDA */
//#define HPDF_COUNTRY_UA  "UA"    /* UKRAINE */
//#define HPDF_COUNTRY_AE  "AE"    /* UNITED ARAB EMIRATES */
//#define HPDF_COUNTRY_GB  "GB"    /* UNITED KINGDOM */
//#define HPDF_COUNTRY_US  "US"    /* UNITED STATES */
//#define HPDF_COUNTRY_UM  "UM"    /* UNITED STATES MINOR OUTLYING ISLANDS */
//#define HPDF_COUNTRY_UY  "UY"    /* URUGUAY */
//#define HPDF_COUNTRY_UZ  "UZ"    /* UZBEKISTAN */
//#define HPDF_COUNTRY_VU  "VU"    /* VANUATU */
//#define HPDF_COUNTRY_VA  "VA"    /* VATICAN CITY STATE (HOLY SEE) */
//#define HPDF_COUNTRY_VE  "VE"    /* VENEZUELA */
//#define HPDF_COUNTRY_VN  "VN"    /* VIET NAM */
//#define HPDF_COUNTRY_VG  "VG"    /* VIRGIN ISLANDS (BRITISH) */
//#define HPDF_COUNTRY_VI  "VI"    /* VIRGIN ISLANDS (U.S.) */
//#define HPDF_COUNTRY_WF  "WF"    /* WALLIS AND FUTUNA ISLANDS */
//#define HPDF_COUNTRY_EH  "EH"    /* WESTERN SAHARA */
//#define HPDF_COUNTRY_YE  "YE"    /* YEMEN */
//#define HPDF_COUNTRY_YU  "YU"    /* YUGOSLAVIA */
//#define HPDF_COUNTRY_ZR  "ZR"    /* ZAIRE */
//#define HPDF_COUNTRY_ZM  "ZM"    /* ZAMBIA */
//#define HPDF_COUNTRY_ZW  "ZW"    /* ZIMBABWE */
//
///*----------------------------------------------------------------------------*/
///*----- lang code definition -------------------------------------------------*/
//
//#define HPDF_LANG_AA    "aa"     /* Afar */
//#define HPDF_LANG_AB    "ab"     /* Abkhazian */
//#define HPDF_LANG_AF    "af"     /* Afrikaans */
//#define HPDF_LANG_AM    "am"     /* Amharic */
//#define HPDF_LANG_AR    "ar"     /* Arabic */
//#define HPDF_LANG_AS    "as"     /* Assamese */
//#define HPDF_LANG_AY    "ay"     /* Aymara */
//#define HPDF_LANG_AZ    "az"     /* Azerbaijani */
//#define HPDF_LANG_BA    "ba"     /* Bashkir */
//#define HPDF_LANG_BE    "be"     /* Byelorussian */
//#define HPDF_LANG_BG    "bg"     /* Bulgarian */
//#define HPDF_LANG_BH    "bh"     /* Bihari */
//#define HPDF_LANG_BI    "bi"     /* Bislama */
//#define HPDF_LANG_BN    "bn"     /* Bengali Bangla */
//#define HPDF_LANG_BO    "bo"     /* Tibetan */
//#define HPDF_LANG_BR    "br"     /* Breton */
//#define HPDF_LANG_CA    "ca"     /* Catalan */
//#define HPDF_LANG_CO    "co"     /* Corsican */
//#define HPDF_LANG_CS    "cs"     /* Czech */
//#define HPDF_LANG_CY    "cy"     /* Welsh */
//#define HPDF_LANG_DA    "da"     /* Danish */
//#define HPDF_LANG_DE    "de"     /* German */
//#define HPDF_LANG_DZ    "dz"     /* Bhutani */
//#define HPDF_LANG_EL    "el"     /* Greek */
//#define HPDF_LANG_EN    "en"     /* English */
//#define HPDF_LANG_EO    "eo"     /* Esperanto */
//#define HPDF_LANG_ES    "es"     /* Spanish */
//#define HPDF_LANG_ET    "et"     /* Estonian */
//#define HPDF_LANG_EU    "eu"     /* Basque */
//#define HPDF_LANG_FA    "fa"     /* Persian */
//#define HPDF_LANG_FI    "fi"     /* Finnish */
//#define HPDF_LANG_FJ    "fj"     /* Fiji */
//#define HPDF_LANG_FO    "fo"     /* Faeroese */
//#define HPDF_LANG_FR    "fr"     /* French */
//#define HPDF_LANG_FY    "fy"     /* Frisian */
//#define HPDF_LANG_GA    "ga"     /* Irish */
//#define HPDF_LANG_GD    "gd"     /* Scots Gaelic */
//#define HPDF_LANG_GL    "gl"     /* Galician */
//#define HPDF_LANG_GN    "gn"     /* Guarani */
//#define HPDF_LANG_GU    "gu"     /* Gujarati */
//#define HPDF_LANG_HA    "ha"     /* Hausa */
//#define HPDF_LANG_HI    "hi"     /* Hindi */
//#define HPDF_LANG_HR    "hr"     /* Croatian */
//#define HPDF_LANG_HU    "hu"     /* Hungarian */
//#define HPDF_LANG_HY    "hy"     /* Armenian */
//#define HPDF_LANG_IA    "ia"     /* Interlingua */
//#define HPDF_LANG_IE    "ie"     /* Interlingue */
//#define HPDF_LANG_IK    "ik"     /* Inupiak */
//#define HPDF_LANG_IN    "in"     /* Indonesian */
//#define HPDF_LANG_IS    "is"     /* Icelandic */
//#define HPDF_LANG_IT    "it"     /* Italian */
//#define HPDF_LANG_IW    "iw"     /* Hebrew */
//#define HPDF_LANG_JA    "ja"     /* Japanese */
//#define HPDF_LANG_JI    "ji"     /* Yiddish */
//#define HPDF_LANG_JW    "jw"     /* Javanese */
//#define HPDF_LANG_KA    "ka"     /* Georgian */
//#define HPDF_LANG_KK    "kk"     /* Kazakh */
//#define HPDF_LANG_KL    "kl"     /* Greenlandic */
//#define HPDF_LANG_KM    "km"     /* Cambodian */
//#define HPDF_LANG_KN    "kn"     /* Kannada */
//#define HPDF_LANG_KO    "ko"     /* Korean */
//#define HPDF_LANG_KS    "ks"     /* Kashmiri */
//#define HPDF_LANG_KU    "ku"     /* Kurdish */
//#define HPDF_LANG_KY    "ky"     /* Kirghiz */
//#define HPDF_LANG_LA    "la"     /* Latin */
//#define HPDF_LANG_LN    "ln"     /* Lingala */
//#define HPDF_LANG_LO    "lo"     /* Laothian */
//#define HPDF_LANG_LT    "lt"     /* Lithuanian */
//#define HPDF_LANG_LV    "lv"     /* Latvian,Lettish */
//#define HPDF_LANG_MG    "mg"     /* Malagasy */
//#define HPDF_LANG_MI    "mi"     /* Maori */
//#define HPDF_LANG_MK    "mk"     /* Macedonian */
//#define HPDF_LANG_ML    "ml"     /* Malayalam */
//#define HPDF_LANG_MN    "mn"     /* Mongolian */
//#define HPDF_LANG_MO    "mo"     /* Moldavian */
//#define HPDF_LANG_MR    "mr"     /* Marathi */
//#define HPDF_LANG_MS    "ms"     /* Malay */
//#define HPDF_LANG_MT    "mt"     /* Maltese */
//#define HPDF_LANG_MY    "my"     /* Burmese */
//#define HPDF_LANG_NA    "na"     /* Nauru */
//#define HPDF_LANG_NE    "ne"     /* Nepali */
//#define HPDF_LANG_NL    "nl"     /* Dutch */
//#define HPDF_LANG_NO    "no"     /* Norwegian */
//#define HPDF_LANG_OC    "oc"     /* Occitan */
//#define HPDF_LANG_OM    "om"     /* (Afan)Oromo */
//#define HPDF_LANG_OR    "or"     /* Oriya */
//#define HPDF_LANG_PA    "pa"     /* Punjabi */
//#define HPDF_LANG_PL    "pl"     /* Polish */
//#define HPDF_LANG_PS    "ps"     /* Pashto,Pushto */
//#define HPDF_LANG_PT    "pt"     /* Portuguese  */
//#define HPDF_LANG_QU    "qu"     /* Quechua */
//#define HPDF_LANG_RM    "rm"     /* Rhaeto-Romance */
//#define HPDF_LANG_RN    "rn"     /* Kirundi */
//#define HPDF_LANG_RO    "ro"     /* Romanian */
//#define HPDF_LANG_RU    "ru"     /* Russian */
//#define HPDF_LANG_RW    "rw"     /* Kinyarwanda */
//#define HPDF_LANG_SA    "sa"     /* Sanskrit */
//#define HPDF_LANG_SD    "sd"     /* Sindhi */
//#define HPDF_LANG_SG    "sg"     /* Sangro */
//#define HPDF_LANG_SH    "sh"     /* Serbo-Croatian */
//#define HPDF_LANG_SI    "si"     /* Singhalese */
//#define HPDF_LANG_SK    "sk"     /* Slovak */
//#define HPDF_LANG_SL    "sl"     /* Slovenian */
//#define HPDF_LANG_SM    "sm"     /* Samoan */
//#define HPDF_LANG_SN    "sn"     /* Shona */
//#define HPDF_LANG_SO    "so"     /* Somali */
//#define HPDF_LANG_SQ    "sq"     /* Albanian */
//#define HPDF_LANG_SR    "sr"     /* Serbian */
//#define HPDF_LANG_SS    "ss"     /* Siswati */
//#define HPDF_LANG_ST    "st"     /* Sesotho */
//#define HPDF_LANG_SU    "su"     /* Sundanese */
//#define HPDF_LANG_SV    "sv"     /* Swedish */
//#define HPDF_LANG_SW    "sw"     /* Swahili */
//#define HPDF_LANG_TA    "ta"     /* Tamil */
//#define HPDF_LANG_TE    "te"     /* Tegulu */
//#define HPDF_LANG_TG    "tg"     /* Tajik */
//#define HPDF_LANG_TH    "th"     /* Thai */
//#define HPDF_LANG_TI    "ti"     /* Tigrinya */
//#define HPDF_LANG_TK    "tk"     /* Turkmen */
//#define HPDF_LANG_TL    "tl"     /* Tagalog */
//#define HPDF_LANG_TN    "tn"     /* Setswanato Tonga */
//#define HPDF_LANG_TR    "tr"     /* Turkish */
//#define HPDF_LANG_TS    "ts"     /* Tsonga */
//#define HPDF_LANG_TT    "tt"     /* Tatar */
//#define HPDF_LANG_TW    "tw"     /* Twi */
//#define HPDF_LANG_UK    "uk"     /* Ukrainian */
//#define HPDF_LANG_UR    "ur"     /* Urdu */
//#define HPDF_LANG_UZ    "uz"     /* Uzbek */
//#define HPDF_LANG_VI    "vi"     /* Vietnamese */
//#define HPDF_LANG_VO    "vo"     /* Volapuk */
//#define HPDF_LANG_WO    "wo"     /* Wolof */
//#define HPDF_LANG_XH    "xh"     /* Xhosa */
//#define HPDF_LANG_YO    "yo"     /* Yoruba */
//#define HPDF_LANG_ZH    "zh"     /* Chinese */
//#define HPDF_LANG_ZU    "zu"     /* Zulu */
//
//
///*----------------------------------------------------------------------------*/
///*----- Graphis mode ---------------------------------------------------------*/
//
//#define   HPDF_GMODE_PAGE_DESCRIPTION       0x0001
//#define   HPDF_GMODE_PATH_OBJECT            0x0002
//#define   HPDF_GMODE_TEXT_OBJECT            0x0004
//#define   HPDF_GMODE_CLIPPING_PATH          0x0008
//#define   HPDF_GMODE_SHADING                0x0010
//#define   HPDF_GMODE_INLINE_IMAGE           0x0020
//#define   HPDF_GMODE_EXTERNAL_OBJECT        0x0040
//
//
///*----------------------------------------------------------------------------*/
//
















/*---------------------------------------------------------------------------*/
/*------ text width struct --------------------------------------------------*/

#[repr(C)]
pub struct HPDF_TextWidth {
    pub numchars: HPDF_UINT,

    /* don't use this value (it may be change in the feature).
       use numspace as alternated. */
    numwords: HPDF_UINT,

    pub width:    HPDF_UINT,
    pub numspace: HPDF_UINT,
} 


/*---------------------------------------------------------------------------*/
/*------ dash mode ----------------------------------------------------------*/

#[repr(C)]
pub struct HPDF_DashMode {
    pub ptn:     [HPDF_UINT16;8],
    pub num_ptn:    HPDF_UINT,
    pub phase:      HPDF_UINT,
} 


/*---------------------------------------------------------------------------*/
/*----- HPDF_TransMatrix struct ---------------------------------------------*/

#[repr(C)]
pub struct HPDF_TransMatrix {
    pub a:      HPDF_REAL, 
    pub b:      HPDF_REAL,   
    pub c:      HPDF_REAL,   
    pub d:      HPDF_REAL,   
    pub x:      HPDF_REAL,   
    pub y:      HPDF_REAL,   
} 

/*---------------------------------------------------------------------------*/
/*----- HPDF_3DMatrix struct ------------------------------------------------*/

#[repr(C)]
pub struct HPDF_3DMatrix {
    pub a:      HPDF_REAL, 
    pub b:      HPDF_REAL, 
    pub c:      HPDF_REAL, 
    pub d:      HPDF_REAL, 
    pub e:      HPDF_REAL, 
    pub f:      HPDF_REAL, 
    pub g:      HPDF_REAL, 
    pub h:      HPDF_REAL, 
    pub i:      HPDF_REAL, 
    pub tx:     HPDF_REAL, 
    pub ty:     HPDF_REAL, 
    pub tz:     HPDF_REAL, 
}


/*---------------------------------------------------------------------------*/

#[repr(C)]
pub enum HPDF_ColorSpace {
    HPDF_CS_DEVICE_GRAY = 0,
    HPDF_CS_DEVICE_RGB,
    HPDF_CS_DEVICE_CMYK,
    HPDF_CS_CAL_GRAY,
    HPDF_CS_CAL_RGB,
    HPDF_CS_LAB,
    HPDF_CS_ICC_BASED,
    HPDF_CS_SEPARATION,
    HPDF_CS_DEVICE_N,
    HPDF_CS_INDEXED,
    HPDF_CS_PATTERN,
    HPDF_CS_EOF
}

/*---------------------------------------------------------------------------*/
/*----- HPDF_RGBColor struct ------------------------------------------------*/

#[repr(C)]
pub struct HPDF_RGBColor {
    pub r:      HPDF_REAL,
    pub g:      HPDF_REAL,   
    pub b:      HPDF_REAL,   
}

/*---------------------------------------------------------------------------*/
/*----- HPDF_CMYKColor struct -----------------------------------------------*/
 
#[repr(C)]
pub struct HPDF_CMYKColor {
    pub c:      HPDF_REAL,   
    pub m:      HPDF_REAL,   
    pub y:      HPDF_REAL,   
    pub k:      HPDF_REAL,   
}

/*---------------------------------------------------------------------------*/
/*------ The line cap style -------------------------------------------------*/

#[repr(C)]
pub enum HPDF_LineCap {
    HPDF_BUTT_END = 0,
    HPDF_ROUND_END,
    HPDF_PROJECTING_SCUARE_END,
    HPDF_LINECAP_EOF
}

/*----------------------------------------------------------------------------*/
/*------ The line join style -------------------------------------------------*/

#[repr(C)]
pub enum HPDF_LineJoin {
    HPDF_MITER_JOIN = 0,
    HPDF_ROUND_JOIN,
    HPDF_BEVEL_JOIN,
    HPDF_LINEJOIN_EOF
}

/*----------------------------------------------------------------------------*/
/*------ The text rendering mode ---------------------------------------------*/

#[repr(C)]
pub enum HPDF_TextRenderingMode {
    HPDF_FILL = 0,
    HPDF_STROKE,
    HPDF_FILL_THEN_STROKE,
    HPDF_INVISIBLE,
    HPDF_FILL_CLIPPING,
    HPDF_STROKE_CLIPPING,
    HPDF_FILL_STROKE_CLIPPING,
    HPDF_CLIPPING,
    HPDF_RENDERING_MODE_EOF
}


#[repr(C)]
pub enum HPDF_WritingMode {
    HPDF_WMODE_HORIZONTAL = 0,
    HPDF_WMODE_VERTICAL,
    HPDF_WMODE_EOF
}


#[repr(C)]
pub enum HPDF_PageLayout {
    HPDF_PAGE_LAYOUT_SINGLE = 0,
    HPDF_PAGE_LAYOUT_ONE_COLUMN,
    HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
    HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
    HPDF_PAGE_LAYOUT_TWO_PAGE_LEFT,
    HPDF_PAGE_LAYOUT_TWO_PAGE_RIGHT,
    HPDF_PAGE_LAYOUT_EOF
}


#[repr(C)]
pub enum HPDF_PageMode {
    HPDF_PAGE_MODE_USE_NONE = 0,
    HPDF_PAGE_MODE_USE_OUTLINE,
    HPDF_PAGE_MODE_USE_THUMBS,
    HPDF_PAGE_MODE_FULL_SCREEN,
/*  HPDF_PAGE_MODE_USE_OC,
    HPDF_PAGE_MODE_USE_ATTACHMENTS,
 */
    HPDF_PAGE_MODE_EOF
}


#[repr(C)]
pub enum HPDF_PageNumStyle {
    HPDF_PAGE_NUM_STYLE_DECIMAL = 0,
    HPDF_PAGE_NUM_STYLE_UPPER_ROMAN,
    HPDF_PAGE_NUM_STYLE_LOWER_ROMAN,
    HPDF_PAGE_NUM_STYLE_UPPER_LETTERS,
    HPDF_PAGE_NUM_STYLE_LOWER_LETTERS,
    HPDF_PAGE_NUM_STYLE_EOF
}


#[repr(C)]
pub enum HPDF_DestinationType {
    HPDF_XYZ = 0,
    HPDF_FIT,
    HPDF_FIT_H,
    HPDF_FIT_V,
    HPDF_FIT_R,
    HPDF_FIT_B,
    HPDF_FIT_BH,
    HPDF_FIT_BV,
    HPDF_DST_EOF
}

#[repr(C)]
pub enum HPDF_AnnotType {
    HPDF_ANNOT_TEXT_NOTES,
    HPDF_ANNOT_LINK,
    HPDF_ANNOT_SOUND,
    HPDF_ANNOT_FREE_TEXT,
    HPDF_ANNOT_STAMP,
    HPDF_ANNOT_SQUARE,
    HPDF_ANNOT_CIRCLE,
    HPDF_ANNOT_STRIKE_OUT,
    HPDF_ANNOT_HIGHTLIGHT,
    HPDF_ANNOT_UNDERLINE,
    HPDF_ANNOT_INK,
    HPDF_ANNOT_FILE_ATTACHMENT,
    HPDF_ANNOT_POPUP,
    HPDF_ANNOT_3D,
    HPDF_ANNOT_SQUIGGLY,
	HPDF_ANNOT_LINE,
	HPDF_ANNOT_PROJECTION,
	HPDF_ANNOT_WIDGET
}


#[repr(C)]
pub enum HPDF_AnnotFlgs {
    HPDF_ANNOT_INVISIBLE,
    HPDF_ANNOT_HIDDEN,
    HPDF_ANNOT_PRINT,
    HPDF_ANNOT_NOZOOM,
    HPDF_ANNOT_NOROTATE,
    HPDF_ANNOT_NOVIEW,
    HPDF_ANNOT_READONLY
}


#[repr(C)]
pub enum HPDF_AnnotHighlightMode {
    HPDF_ANNOT_NO_HIGHTLIGHT = 0,
    HPDF_ANNOT_INVERT_BOX,
    HPDF_ANNOT_INVERT_BORDER,
    HPDF_ANNOT_DOWN_APPEARANCE,
    HPDF_ANNOT_HIGHTLIGHT_MODE_EOF
}


#[repr(C)]
pub enum HPDF_AnnotIcon {
    HPDF_ANNOT_ICON_COMMENT = 0,
    HPDF_ANNOT_ICON_KEY,
    HPDF_ANNOT_ICON_NOTE,
    HPDF_ANNOT_ICON_HELP,
    HPDF_ANNOT_ICON_NEW_PARAGRAPH,
    HPDF_ANNOT_ICON_PARAGRAPH,
    HPDF_ANNOT_ICON_INSERT,
    HPDF_ANNOT_ICON_EOF
}

#[repr(C)]
pub enum HPDF_AnnotIntent {
    HPDF_ANNOT_INTENT_FREETEXTCALLOUT = 0,
    HPDF_ANNOT_INTENT_FREETEXTTYPEWRITER,
    HPDF_ANNOT_INTENT_LINEARROW,
    HPDF_ANNOT_INTENT_LINEDIMENSION,
    HPDF_ANNOT_INTENT_POLYGONCLOUD,
    HPDF_ANNOT_INTENT_POLYLINEDIMENSION,
    HPDF_ANNOT_INTENT_POLYGONDIMENSION
}

#[repr(C)]
pub enum HPDF_LineAnnotEndingStyle {
    HPDF_LINE_ANNOT_NONE = 0,
    HPDF_LINE_ANNOT_SQUARE,
    HPDF_LINE_ANNOT_CIRCLE,
    HPDF_LINE_ANNOT_DIAMOND,
    HPDF_LINE_ANNOT_OPENARROW,
    HPDF_LINE_ANNOT_CLOSEDARROW,
    HPDF_LINE_ANNOT_BUTT,
    HPDF_LINE_ANNOT_ROPENARROW,
    HPDF_LINE_ANNOT_RCLOSEDARROW,
    HPDF_LINE_ANNOT_SLASH
}

#[repr(C)]
pub enum HPDF_LineAnnotCapPosition{
    HPDF_LINE_ANNOT_CAP_INLINE = 0,
    HPDF_LINE_ANNOT_CAP_TOP
}

#[repr(C)]
pub enum HPDF_StampAnnotName{
    HPDF_STAMP_ANNOT_APPROVED = 0,
    HPDF_STAMP_ANNOT_EXPERIMENTAL,
    HPDF_STAMP_ANNOT_NOTAPPROVED,
    HPDF_STAMP_ANNOT_ASIS,
    HPDF_STAMP_ANNOT_EXPIRED,
    HPDF_STAMP_ANNOT_NOTFORPUBLICRELEASE,
    HPDF_STAMP_ANNOT_CONFIDENTIAL,
    HPDF_STAMP_ANNOT_FINAL,
    HPDF_STAMP_ANNOT_SOLD,
    HPDF_STAMP_ANNOT_DEPARTMENTAL,
    HPDF_STAMP_ANNOT_FORCOMMENT,
    HPDF_STAMP_ANNOT_TOPSECRET,
    HPDF_STAMP_ANNOT_DRAFT,
    HPDF_STAMP_ANNOT_FORPUBLICRELEASE
}

/*----------------------------------------------------------------------------*/
/*------ border stype --------------------------------------------------------*/

#[repr(C)]
pub enum HPDF_BSSubtype {
    HPDF_BS_SOLID,
    HPDF_BS_DASHED,
    HPDF_BS_BEVELED,
    HPDF_BS_INSET,
    HPDF_BS_UNDERLINED
}


/*----- blend modes ----------------------------------------------------------*/

#[repr(C)]
pub enum HPDF_BlendMode {
    HPDF_BM_NORMAL,
    HPDF_BM_MULTIPLY,
    HPDF_BM_SCREEN,
    HPDF_BM_OVERLAY,
    HPDF_BM_DARKEN,
    HPDF_BM_LIGHTEN,
    HPDF_BM_COLOR_DODGE,
    HPDF_BM_COLOR_BUM,
    HPDF_BM_HARD_LIGHT,
    HPDF_BM_SOFT_LIGHT,
    HPDF_BM_DIFFERENCE,
    HPDF_BM_EXCLUSHON,
    HPDF_BM_EOF
}

/*----- slide show -----------------------------------------------------------*/

#[repr(C)]
pub enum HPDF_TransitionStyle {
    HPDF_TS_WIPE_RIGHT = 0,
    HPDF_TS_WIPE_UP,
    HPDF_TS_WIPE_LEFT,
    HPDF_TS_WIPE_DOWN,
    HPDF_TS_BARN_DOORS_HORIZONTAL_OUT,
    HPDF_TS_BARN_DOORS_HORIZONTAL_IN,
    HPDF_TS_BARN_DOORS_VERTICAL_OUT,
    HPDF_TS_BARN_DOORS_VERTICAL_IN,
    HPDF_TS_BOX_OUT,
    HPDF_TS_BOX_IN,
    HPDF_TS_BLINDS_HORIZONTAL,
    HPDF_TS_BLINDS_VERTICAL,
    HPDF_TS_DISSOLVE,
    HPDF_TS_GLITTER_RIGHT,
    HPDF_TS_GLITTER_DOWN,
    HPDF_TS_GLITTER_TOP_LEFT_TO_BOTTOM_RIGHT,
    HPDF_TS_REPLACE,
    HPDF_TS_EOF
}

/*----------------------------------------------------------------------------*/

#[repr(C)]
pub enum HPDF_PageSizes {
    HPDF_PAGE_SIZE_LETTER = 0,
    HPDF_PAGE_SIZE_LEGAL,
    HPDF_PAGE_SIZE_A3,
    HPDF_PAGE_SIZE_A4,
    HPDF_PAGE_SIZE_A5,
    HPDF_PAGE_SIZE_B4,
    HPDF_PAGE_SIZE_B5,
    HPDF_PAGE_SIZE_EXECUTIVE,
    HPDF_PAGE_SIZE_US4x6,
    HPDF_PAGE_SIZE_US4x8,
    HPDF_PAGE_SIZE_US5x7,
    HPDF_PAGE_SIZE_COMM10,
    HPDF_PAGE_SIZE_EOF
}


#[repr(C)]
pub enum HPDF_PageDirection {
    HPDF_PAGE_PORTRAIT = 0,
    HPDF_PAGE_LANDSCAPE
}


#[repr(C)]
pub enum  HPDF_EncoderType {
    HPDF_ENCODER_TYPE_SINGLE_BYTE,
    HPDF_ENCODER_TYPE_DOUBLE_BYTE,
    HPDF_ENCODER_TYPE_UNINITIALIZED,
    HPDF_ENCODER_UNKNOWN
}


#[repr(C)]
pub enum HPDF_ByteType {
    HPDF_BYTE_TYPE_SINGLE = 0,
    HPDF_BYTE_TYPE_LEAD,
    HPDF_BYTE_TYPE_TRIAL,
    HPDF_BYTE_TYPE_UNKNOWN
}


#[repr(C)]
pub enum HPDF_TextAlignment {
    HPDF_TALIGN_LEFT = 0,
    HPDF_TALIGN_RIGHT,
    HPDF_TALIGN_CENTER,
    HPDF_TALIGN_JUSTIFY
}

/*----------------------------------------------------------------------------*/

/* Name Dictionary values -- see PDF reference section 7.7.4 */
#[repr(C)]
pub enum _HPDF_NameDictKey {
    HPDF_NAME_EMBEDDED_FILES = 0,    /* TODO the rest */
    HPDF_NAME_EOF
}



pub type HPDF_HANDLE   = *mut libc::c_void;
pub type HPDF_Doc       =   HPDF_HANDLE;
pub type HPDF_Page      =   HPDF_HANDLE; 
pub type HPDF_Pages     =   HPDF_HANDLE;
pub type HPDF_Stream    =   HPDF_HANDLE;
pub type HPDF_Image     =   HPDF_HANDLE;
pub type HPDF_Font      =   HPDF_HANDLE;
pub type HPDF_Outline   =   HPDF_HANDLE;
pub type HPDF_Encoder   =   HPDF_HANDLE;
pub type HPDF_3DMeasure =   HPDF_HANDLE;
pub type HPDF_ExData    =   HPDF_HANDLE;
pub type HPDF_Destination=  HPDF_HANDLE;
pub type HPDF_XObject   =   HPDF_HANDLE;
pub type HPDF_Annotation=   HPDF_HANDLE;
pub type HPDF_ExtGState =   HPDF_HANDLE;
pub type HPDF_FontDef   =   HPDF_HANDLE;
pub type HPDF_U3D       =   HPDF_HANDLE;
pub type HPDF_JavaScript=   HPDF_HANDLE;
pub type HPDF_Error     =   HPDF_HANDLE;
pub type HPDF_MMgr      =   HPDF_HANDLE;
pub type HPDF_Dict      =   HPDF_HANDLE;
pub type HPDF_EmbeddedFile= HPDF_HANDLE;
pub type HPDF_OutputIntent= HPDF_HANDLE;
pub type HPDF_Xref      =   HPDF_HANDLE;






extern{
    pub fn HPDF_New  (user_error_fn: extern fn(HPDF_STATUS, HPDF_STATUS, HPDF_HANDLE), user_data: HPDF_HANDLE) -> HPDF_Doc;


    pub fn HPDF_Free(pdf: HPDF_Doc)->libc::c_void;

    pub fn HPDF_SaveToFile(pdf: HPDF_Doc,
                           file_name:  *const libc::c_char)->HPDF_STATUS;



/*--- Text object operator -----------------------------------------------*/

/* BT */
    pub fn HPDF_Page_BeginText  (page: HPDF_Page)->HPDF_STATUS;

/* ET */
    pub fn HPDF_Page_EndText  (page: HPDF_Page)->HPDF_STATUS;

/*--- Text state ---------------------------------------------------------*/

/* Tc */
    pub fn HPDF_Page_SetCharSpace  (page:   HPDF_Page,
                                     value : HPDF_REAL)->HPDF_STATUS;
//
//* Tw */
    pub fn HPDF_Page_SetWordSpace  (page : HPDF_Page, 
                                    value: HPDF_REAL, )->HPDF_STATUS;
//
//* Tz */
    pub fn HPDF_Page_SetHorizontalScalling  ( page: HPDF_Page,
                                              value: HPDF_REAL )->HPDF_STATUS;
//
//* TL */
    pub fn HPDF_Page_SetTextLeading  (page : HPDF_Page,
                                      value: HPDF_REAL )->HPDF_STATUS;
//
//* Tf */
    pub fn HPDF_Page_SetFontAndSize  (page:  HPDF_Page ,
                                       font:  HPDF_Font,
                                       size:  HPDF_REAL)->HPDF_STATUS;

//* Tr */
    pub fn HPDF_Page_SetTextRenderingMode  (page: HPDF_Page,
                                          mode: HPDF_TextRenderingMode)->HPDF_STATUS;
//
//* Ts */
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Page_SetTextRise  (HPDF_Page   page,
//                        HPDF_REAL   value);
//
//* This function is obsolete. Use HPDF_Page_SetTextRise.  */
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Page_SetTextRaise  (HPDF_Page   page,
//                         HPDF_REAL   value);
//
//
/*--- Text positioning ---------------------------------------------------*/

/* Td */
    pub fn HPDF_Page_MoveTextPos(page:  HPDF_Page,
                                 x:     HPDF_REAL,
                                 y:     HPDF_REAL)->HPDF_STATUS;

/* TD */
    pub fn HPDF_Page_MoveTextPos2  ( page:  HPDF_Page,
                                     x:     HPDF_REAL,
                                     y:     HPDF_REAL)->HPDF_STATUS;

/* Tm */
    pub fn HPDF_Page_SetTextMatrix  ( page:    HPDF_Page,
                                      a:    HPDF_REAL,
                                      b:    HPDF_REAL,
                                      c:    HPDF_REAL,
                                      d:    HPDF_REAL,
                                      x:    HPDF_REAL,
                                      y:    HPDF_REAL)->HPDF_STATUS;


/* T* */
    pub fn HPDF_Page_MoveToNextLine  (page: HPDF_Page)->HPDF_STATUS;
/*--- Text showing -------------------------------------------------------*/

/* Tj */
    pub fn HPDF_Page_ShowText(page: HPDF_Page,
                             text: *const libc::c_char)->HPDF_STATUS;

/* TJ */

/* ' */
    pub fn HPDF_Page_ShowTextNextLine (page: HPDF_Page,                       
                                   text: *const libc::c_char)->HPDF_STATUS; 

/* " */
    pub fn HPDF_Page_ShowTextNextLineEx(page      :  HPDF_Page,
                                        word_space: HPDF_REAL,
                                        char_space: HPDF_REAL,
                                        text      :  *const libc::c_char )->HPDF_STATUS;

/*--- Color showing ------------------------------------------------------*/

/* cs --not implemented yet */
/* CS --not implemented yet */
/* sc --not implemented yet */
/* scn --not implemented yet */
/* SC --not implemented yet */
/* SCN --not implemented yet */

/* g */
    pub fn HPDF_Page_SetGrayFill  (page: HPDF_Page ,
                                  gray: HPDF_REAL)->HPDF_STATUS;

/* G */
    pub fn HPDF_Page_SetGrayStroke  (page: HPDF_Page  ,
                                    gray: HPDF_REAL   )->HPDF_STATUS;

/* rg */
    pub fn HPDF_Page_SetRGBFill  (page: HPDF_Page,
                                   r   : HPDF_REAL,
                                   g   : HPDF_REAL,
                                   b   : HPDF_REAL)->HPDF_STATUS;

/* RG */
    pub fn HPDF_Page_SetRGBStroke  (page: HPDF_Page,   
                                    r:    HPDF_REAL,  
                                    g:    HPDF_REAL,  
                                    b:    HPDF_REAL,  )->HPDF_STATUS;

/* k */
    pub fn HPDF_Page_SetCMYKFill  (page: HPDF_Page,   
                                   c:    HPDF_REAL,  
                                   m:    HPDF_REAL,  
                                   y:    HPDF_REAL,  
                                   k:    HPDF_REAL,  )->HPDF_STATUS;

/* K */
    pub fn HPDF_Page_SetCMYKStroke  (page: HPDF_Page,  
                                     c:    HPDF_REAL, 
                                     m:    HPDF_REAL, 
                                     y:    HPDF_REAL, 
                                     k:    HPDF_REAL,    )->HPDF_STATUS;


/*--------------------------------------------------------------------------*/
/*----- GRAPHICS OPERATORS -------------------------------------------------*/


/*--- General graphics state ---------------------------------------------*/

/* w */
    pub fn HPDF_Page_SetLineWidth  (page: HPDF_Page,
                                    line_width: HPDF_REAL)->HPDF_STATUS;

/* J */
    pub fn HPDF_Page_SetLineCap  (  page    : HPDF_Page,
                                    line_cap: HPDF_LineCap)->HPDF_STATUS;

///* j */
    pub fn HPDF_Page_SetLineJoin(page       : HPDF_Page ,
                                 line_join  : HPDF_LineJoin)->HPDF_STATUS;

///* M */
    pub fn HPDF_Page_SetMiterLimit  (page: HPDF_Page,
                                      miter_limit: HPDF_REAL)->HPDF_STATUS;
//
///* d */
    pub fn HPDF_Page_SetDash  (  page      :  HPDF_Page,       
                                  dash_ptn  : *const HPDF_UINT16,
                                  num_param : HPDF_UINT, 
                                  phase     : HPDF_UINT         )->HPDF_STATUS;



/* ri --not implemented yet */

///* i */
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Page_SetFlat  (HPDF_Page    page,
//                    HPDF_REAL    flatness);
//
///* gs */

    pub fn HPDF_Page_SetExtGState  (page       : HPDF_Page ,
                                    ext_gstate : HPDF_ExtGState )->HPDF_STATUS;


///*--- Special graphic state operator --------------------------------------*/
//
///* q */
    pub fn HPDF_Page_GSave  (page: HPDF_Page)->HPDF_STATUS;
//
///* Q */
    pub fn HPDF_Page_GRestore  (page: HPDF_Page)->HPDF_STATUS;
//
///* cm */
    pub fn HPDF_Page_Concat  (page  :   HPDF_Page,    
                               a     :   HPDF_REAL,    
                               b     :   HPDF_REAL,    
                               c     :   HPDF_REAL,    
                               d     :   HPDF_REAL,    
                               x     :   HPDF_REAL,    
                               y     :   HPDF_REAL  )->HPDF_STATUS;

/*--- Path construction operator ------------------------------------------*/

/* m */
    pub fn HPDF_Page_MoveTo  (page : HPDF_Page,
                               x   : HPDF_REAL,
                               y   : HPDF_REAL)->HPDF_STATUS;

//* l */
    pub fn HPDF_Page_LineTo  (page: HPDF_Page,    
                               x:   HPDF_REAL,    
                               y:    HPDF_REAL, )->HPDF_STATUS;

//* c */
    pub fn HPDF_Page_CurveTo  (page : HPDF_Page,
                                x1   : HPDF_REAL,  
                                y1   : HPDF_REAL,  
                                x2   : HPDF_REAL,  
                                y2   : HPDF_REAL,  
                                x3   : HPDF_REAL,  
                                y3   : HPDF_REAL   )->HPDF_STATUS;

//* v */
//HPDF_EXPORT(HPDF_STATUS)
    pub fn HPDF_Page_CurveTo2  (page    : HPDF_Page, 
                                 x2      : HPDF_REAL, 
                                 y2      : HPDF_REAL, 
                                 x3      : HPDF_REAL, 
                                 y3      : HPDF_REAL  )->HPDF_STATUS;

//* y */
    pub fn HPDF_Page_CurveTo3  (page    : HPDF_Page,  
                                 x1      : HPDF_REAL,  
                                 y1      : HPDF_REAL,  
                                 x3      : HPDF_REAL,  
                                 y3      : HPDF_REAL  )->HPDF_STATUS;

//* h */
//HPDF_EXPORT(HPDF_STATUS)
    pub fn HPDF_Page_ClosePath  (page: HPDF_Page)->HPDF_STATUS;

/* re */
    pub fn HPDF_Page_Rectangle  (page   : HPDF_Page,
                                  x     : HPDF_REAL,
                                  y     : HPDF_REAL,
                                  width : HPDF_REAL,
                                  height: HPDF_REAL)->HPDF_STATUS;


/*--- Path painting operator ---------------------------------------------*/

/* S */
    pub fn HPDF_Page_Stroke  (page: HPDF_Page)->HPDF_STATUS;

/* s */
    pub fn HPDF_Page_ClosePathStroke  (page: HPDF_Page)->HPDF_STATUS;
//
//* f */
    pub fn HPDF_Page_Fill  (page: HPDF_Page)->HPDF_STATUS;

//* f* */
    pub fn HPDF_Page_Eofill  (page: HPDF_Page )->HPDF_STATUS;

//* B */
    pub fn HPDF_Page_FillStroke  (page: HPDF_Page)->HPDF_STATUS;

//* B* */
    pub fn HPDF_Page_EofillStroke  (page: HPDF_Page)->HPDF_STATUS;

//* b */
    pub fn HPDF_Page_ClosePathFillStroke  (page: HPDF_Page)->HPDF_STATUS;

//* b* */
    pub fn HPDF_Page_ClosePathEofillStroke  (page: HPDF_Page)->HPDF_STATUS;

//* n */
    pub fn HPDF_Page_EndPath  (page: HPDF_Page)->HPDF_STATUS;
/*--- Clipping paths operator --------------------------------------------*/

/* W */
    pub fn HPDF_Page_Clip  (page: HPDF_Page)->HPDF_STATUS;

/* W* */
    pub fn HPDF_Page_Eoclip  (page: HPDF_Page)->HPDF_STATUS;
/*--------------------------------------------------------------------------*/
/*----- encryption ---------------------------------------------------------*/

    pub fn  HPDF_SetPassword  (pdf          :   HPDF_Doc,
                               owner_passwd : *const libc::c_char,
                               user_passwd  : *const libc::c_char)->HPDF_STATUS;


    pub fn  HPDF_SetPermission  (pdf    :   HPDF_Doc,
                                 permission : HPDF_UINT)->HPDF_STATUS;


    pub fn  HPDF_SetEncryptionMode  ( pdf       :   HPDF_Doc,       
                                      mode      :   HPDF_EncryptMode, 
                                      key_len   :   HPDF_UINT         )->HPDF_STATUS;


/*--------------------------------------------------------------------------*/
/*----- compression --------------------------------------------------------*/

    pub fn HPDF_SetCompressionMode( pdf : HPDF_Doc,
                                    mode: HPDF_UINT )->HPDF_STATUS;

/*--------------------------------------------------------------------------*/
/*----- font ---------------------------------------------------------------*/

    pub fn HPDF_Font_GetFontName  (font : HPDF_Font )->*const libc::c_char;


    pub fn HPDF_Font_GetEncodingName  (font : HPDF_Font )->*const libc::c_char;


    pub fn HPDF_Font_GetUnicodeWidth  (font : HPDF_Font,
                                       code : HPDF_UNICODE)->HPDF_INT;

    pub fn HPDF_Font_GetBBox  (font : HPDF_Font)->HPDF_Box;


    pub fn HPDF_Font_GetAscent  (font : HPDF_Font)->HPDF_INT;


    pub fn HPDF_Font_GetDescent  (font : HPDF_Font )->HPDF_INT;


    pub fn HPDF_Font_GetXHeight  (font : HPDF_Font)->HPDF_UINT;


    pub fn HPDF_Font_GetCapHeight  (font : HPDF_Font)->HPDF_UINT;


    pub fn  HPDF_Font_TextWidth  (font : HPDF_Font,
                                  text : *const libc::c_char,
                                  len  : HPDF_UINT           )->HPDF_TextWidth;


//HPDF_EXPORT(HPDF_UINT)
//HPDF_Font_MeasureText (HPDF_Font          font,
//                       const HPDF_BYTE   *text,
//                       HPDF_UINT          len,
//                       HPDF_REAL          width,
//                       HPDF_REAL          font_size,
//                       HPDF_REAL          char_space,
//                       HPDF_REAL          word_space,
//                       HPDF_BOOL          wordwrap,
//                       HPDF_REAL         *real_width);
//

/*--------------------------------------------------------------------------*/
/*----- attachements -------------------------------------------------------*/

    pub fn HPDF_AttachFile  (pdf:  HPDF_Doc ,
                              file: *const libc::c_char ) -> HPDF_EmbeddedFile;

/*--------------------------------------------------------------------------*/
/*----- extended graphics state --------------------------------------------*/

    pub fn HPDF_CreateExtGState  (pdf: HPDF_Doc )->HPDF_ExtGState;


    pub fn HPDF_ExtGState_SetAlphaStroke  ( ext_gstate  :  HPDF_ExtGState,
                                             value      :  HPDF_REAL         )->HPDF_STATUS;


    pub fn HPDF_ExtGState_SetAlphaFill  (ext_gstate : HPDF_ExtGState,
                                         value      :  HPDF_REAL ) ->HPDF_STATUS;



    pub fn HPDF_ExtGState_SetBlendMode  (ext_gstate : HPDF_ExtGState,
                                         mode       : HPDF_BlendMode)->HPDF_STATUS;


/*--------------------------------------------------------------------------*/
/*--------------------------------------------------------------------------*/

    pub fn HPDF_Page_TextWidth( page: HPDF_Page,
                      text: *const libc::c_char )-> HPDF_REAL;


    pub fn HPDF_Page_MeasureText  (page: HPDF_Page,
                        text: *const libc::c_char,
                        width: HPDF_REAL,
                        wordwrap: HPDF_BOOL,
                        real_width: *mut HPDF_REAL)-> HPDF_UINT;


    pub fn HPDF_Page_GetWidth  (page: HPDF_Page)-> HPDF_REAL;


    pub fn HPDF_Page_GetHeight (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetGMode  (page: HPDF_Page)-> HPDF_UINT16;


    pub fn HPDF_Page_GetCurrentPos (page: HPDF_Page)-> HPDF_Point;


    pub fn HPDF_Page_GetCurrentPos2  (page: HPDF_Page,
                                      pos: *mut HPDF_Point)->HPDF_STATUS;


    pub fn HPDF_Page_GetCurrentTextPos (page: HPDF_Page)->HPDF_Point;


    pub fn HPDF_Page_GetCurrentTextPos2  (page: HPDF_Page,
                                          pos: *mut HPDF_Point)->HPDF_STATUS;


    pub fn HPDF_Page_GetCurrentFont  (page: HPDF_Page)->HPDF_Font;


    pub fn HPDF_Page_GetCurrentFontSize  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetTransMatrix  (page: HPDF_Page)->HPDF_TransMatrix;


    pub fn HPDF_Page_GetLineWidth  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetLineCap  (page: HPDF_Page)->HPDF_LineCap;


    pub fn HPDF_Page_GetLineJoin  (page: HPDF_Page)->HPDF_LineJoin;


    pub fn HPDF_Page_GetMiterLimit  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetDash  (page: HPDF_Page)->HPDF_DashMode;


    pub fn HPDF_Page_GetFlat  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetCharSpace  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetWordSpace  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetHorizontalScalling  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetTextLeading  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetTextRenderingMode  (page: HPDF_Page)->HPDF_TextRenderingMode;


/* This function is obsolete. Use HPDF_Page_GetTextRise.  */
////HPDF_EXPORT(XXXX)
////HPDF_Page_GetTextRaise  (XXXX   XXXX);


    pub fn HPDF_Page_GetTextRise  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetRGBFill  (page: HPDF_Page)->HPDF_RGBColor;


    pub fn HPDF_Page_GetRGBStroke  (page: HPDF_Page)->HPDF_RGBColor;


    pub fn HPDF_Page_GetCMYKFill  (page: HPDF_Page)->HPDF_CMYKColor;


    pub fn HPDF_Page_GetCMYKStroke  (page: HPDF_Page)->HPDF_CMYKColor;


    pub fn HPDF_Page_GetGrayFill  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetGrayStroke  (page: HPDF_Page)->HPDF_REAL;


    pub fn HPDF_Page_GetStrokingColorSpace (page: HPDF_Page)->HPDF_ColorSpace;


    pub fn HPDF_Page_GetFillingColorSpace (page: HPDF_Page)->HPDF_ColorSpace;


    pub fn HPDF_Page_GetTextMatrix  (page: HPDF_Page)->HPDF_TransMatrix;


    pub fn HPDF_Page_GetGStateDepth  (page: HPDF_Page)->HPDF_UINT;
/*---------------------------------------------------------------------------*/
/*---------------------------------------------------------------------------*/

//HPDF_EXPORT(HPDF_MMgr)
//HPDF_GetPageMMgr    (HPDF_Page  page);
//
//HPDF_EXPORT(HPDF_PageLayout)
//HPDF_GetPageLayout  (HPDF_Doc   pdf);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_SetPageLayout  (HPDF_Doc          pdf,
//                     HPDF_PageLayout   layout);
//
//
//HPDF_EXPORT(HPDF_PageMode)
//HPDF_GetPageMode  (HPDF_Doc   pdf);
//
//
    pub fn HPDF_SetPageMode  (pdf  : HPDF_Doc,
                              mode : HPDF_PageMode )->HPDF_STATUS;


//HPDF_EXPORT(HPDF_UINT)
//HPDF_GetViewerPreference  (HPDF_Doc   pdf);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_SetViewerPreference  (HPDF_Doc     pdf,
//                           HPDF_UINT    value);
//
//
    pub fn HPDF_SetOpenAction  (pdf: HPDF_Doc ,
                                 open_action: HPDF_Destination)->HPDF_STATUS;


/*---------------------------------------------------------------------------*/
/*----- page handling -------------------------------------------------------*/


    pub fn HPDF_GetCurrentPage  (pdf: HPDF_Doc)->HPDF_Page;
 
 
    pub fn HPDF_AddPage  (pdf: HPDF_Doc )->HPDF_Page;


    pub fn HPDF_InsertPage  (pdf:  HPDF_Doc  ,
                             page:  HPDF_Page)->HPDF_Page;


    pub fn HPDF_Page_SetWidth  (page : HPDF_Page ,
                                vale : HPDF_REAL )->HPDF_STATUS;


    pub fn HPDF_Page_SetHeight  (page:  HPDF_Page ,
                                  value: HPDF_REAL)->HPDF_STATUS;


    pub fn HPDF_Page_SetSize  (page      :    HPDF_Page         ,   
                               size      :    HPDF_PageSizes    ,   
                               direction : HPDF_PageDirection   )->HPDF_STATUS;


    pub fn HPDF_Page_SetRotate  (page:   HPDF_Page ,
                                 angle : HPDF_UINT16  )->HPDF_STATUS;


    pub fn HPDF_Page_SetZoom  (page    : HPDF_Page ,
                               zoom    : HPDF_REAL     )->HPDF_STATUS;


/*---------------------------------------------------------------------------*/
/*----- font handling -------------------------------------------------------*/


    pub fn HPDF_GetFont  (pdf: HPDF_Doc,
                           font_name: *const libc::c_char,
                           encoding_name: *const libc::c_char)->HPDF_Font;
    

//HPDF_EXPORT(const char*)
//HPDF_LoadType1FontFromFile  (HPDF_Doc     pdf,
//                             const char  *afm_file_name,
//                             const char  *data_file_name);
//
//
//HPDF_EXPORT(HPDF_FontDef)
//HPDF_GetTTFontDefFromFile (HPDF_Doc     pdf,
//                           const char  *file_name,
//                           HPDF_BOOL    embedding);
//
    pub fn HPDF_LoadTTFontFromFile (pdf: HPDF_Doc,
                                     file_name: *const libc::c_char,
                                     embedding: HPDF_BOOL)-> *const libc::c_char;

//
//HPDF_EXPORT(const char*)
//HPDF_LoadTTFontFromFile2 (HPDF_Doc     pdf,
//                          const char  *file_name,
//                          HPDF_UINT    index,
//                          HPDF_BOOL    embedding);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_AddPageLabel  (HPDF_Doc            pdf,
//                    HPDF_UINT           page_num,
//                    HPDF_PageNumStyle   style,
//                    HPDF_UINT           first_page,
//                    const char         *prefix);
//
//
    pub fn HPDF_UseJPFonts   (pdf : HPDF_Doc)->HPDF_STATUS;


    pub fn HPDF_UseKRFonts   (pdf : HPDF_Doc)->HPDF_STATUS;


    pub fn HPDF_UseCNSFonts   (pdf : HPDF_Doc )->HPDF_STATUS;


    pub fn HPDF_UseCNTFonts   (pdf : HPDF_Doc  )->HPDF_STATUS;

/*--------------------------------------------------------------------------*/
/*----- outline ------------------------------------------------------------*/


    pub fn  HPDF_CreateOutline  (pdf     : HPDF_Doc    ,
                                 parent  : HPDF_Outline,
                                 title   : *const libc::c_char,
                                 encoder : HPDF_Encoder    )->HPDF_Outline;


    pub fn  HPDF_Outline_SetOpened  (outline : HPDF_Outline,
                                     opened  : HPDF_BOOL)->HPDF_STATUS;


    pub fn  HPDF_Outline_SetDestination (outline : HPDF_Outline,
                                         dst : HPDF_Destination )->HPDF_STATUS;


/*--------------------------------------------------------------------------*/
/*----- destination --------------------------------------------------------*/

    pub fn HPDF_Page_CreateDestination (page: HPDF_Page)->HPDF_Destination ;


    pub fn HPDF_Destination_SetXYZ  (dst  : HPDF_Destination, 
                                      left : HPDF_REAL,        
                                      top  : HPDF_REAL,        
                                      zoom : HPDF_REAL)->HPDF_STATUS;


//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFit  (HPDF_Destination  dst);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitH  (HPDF_Destination  dst,
//                           HPDF_REAL         top);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitV  (HPDF_Destination  dst,
//                           HPDF_REAL         left);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitR  (HPDF_Destination  dst,
//                           HPDF_REAL         left,
//                           HPDF_REAL         bottom,
//                           HPDF_REAL         right,
//                           HPDF_REAL         top);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitB  (HPDF_Destination  dst);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitBH  (HPDF_Destination  dst,
//                            HPDF_REAL         top);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Destination_SetFitBV  (HPDF_Destination  dst,
//                            HPDF_REAL         left);

/*--------------------------------------------------------------------------*/
/*----- encoder ------------------------------------------------------------*/

    pub fn HPDF_GetEncoder  (pdf           : HPDF_Doc,
                             encoding_name : *const libc::c_char)->HPDF_Encoder;


    pub fn HPDF_GetCurrentEncoder  (pdf : HPDF_Doc)->HPDF_Encoder;


    pub fn HPDF_SetCurrentEncoder  (pdf           : HPDF_Doc,
                                    encoding_name : *const libc::c_char)->HPDF_STATUS;


    pub fn HPDF_Encoder_GetType  (encoder : HPDF_Encoder  )->HPDF_EncoderType;


//HPDF_EXPORT(HPDF_ByteType)
//HPDF_Encoder_GetByteType  (HPDF_Encoder    encoder,
//                           const char     *text,
//                           HPDF_UINT       index);
//
//
    pub fn HPDF_Encoder_GetUnicode  (encoder : HPDF_Encoder ,
                                     code : HPDF_UINT16 )->HPDF_UNICODE;


    pub fn HPDF_Encoder_GetWritingMode (encoder : HPDF_Encoder )->HPDF_WritingMode;


    pub fn HPDF_UseJPEncodings   (pdf : HPDF_Doc  )->HPDF_STATUS;



    pub fn HPDF_UseKREncodings   (pdf : HPDF_Doc )->HPDF_STATUS;



    pub fn HPDF_UseCNSEncodings   (pdf : HPDF_Doc )->HPDF_STATUS;



    pub fn HPDF_UseCNTEncodings   (pdf : HPDF_Doc )->HPDF_STATUS;


    pub fn HPDF_UseUTFEncodings   (pdf : HPDF_Doc)->HPDF_STATUS;


    
/*--------------------------------------------------------------------------*/
/*----- annotation ---------------------------------------------------------*/

//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_Create3DAnnot    (HPDF_Page       page,
//							HPDF_Rect       rect,
//                            HPDF_BOOL       tb,
//                            HPDF_BOOL       np,
//                            HPDF_U3D        u3d,
//                            HPDF_Image      ap);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateTextAnnot  (HPDF_Page       page,
//                            HPDF_Rect       rect,
//                            const char     *text,
//                            HPDF_Encoder    encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateFreeTextAnnot  (HPDF_Page       page,
//								HPDF_Rect       rect,
//								const char     *text,
//								HPDF_Encoder    encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateLineAnnot  (HPDF_Page       page,
//							const char     *text,
//							HPDF_Encoder    encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateWidgetAnnot_WhiteOnlyWhilePrint (HPDF_Doc   pdf,
//                                                 HPDF_Page  page,
//                                                 HPDF_Rect  rect);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateWidgetAnnot (HPDF_Page  page,
//                             HPDF_Rect  rect);
//
    pub fn HPDF_Page_CreateLinkAnnot  (page : HPDF_Page          ,
                                        rect : HPDF_Rect          ,
                                        dst  : HPDF_Destination    )->HPDF_Annotation ;


    pub fn HPDF_Page_CreateURILinkAnnot  (page: HPDF_Page,
                                           rect: HPDF_Rect,
                                           uri : *const libc::c_char)->HPDF_Annotation;


//HPDF_Annotation
//HPDF_Page_CreateTextMarkupAnnot (HPDF_Page     page,
//								HPDF_Rect      rect,
//								const char     *text,
//								HPDF_Encoder   encoder,
//								HPDF_AnnotType subType);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateHighlightAnnot  (HPDF_Page   page,
//								HPDF_Rect    rect,
//								const char   *text,
//								HPDF_Encoder encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateUnderlineAnnot (HPDF_Page    page,
//								HPDF_Rect    rect,
//								const char   *text,
//								HPDF_Encoder encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateSquigglyAnnot  (HPDF_Page    page,
//								HPDF_Rect    rect,
//								const char   *text,
//								HPDF_Encoder encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateStrikeOutAnnot  (HPDF_Page   page,
//								HPDF_Rect    rect,
//								const char   *text,
//								HPDF_Encoder encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreatePopupAnnot  (	HPDF_Page          page,
//								HPDF_Rect          rect,
//								HPDF_Annotation	   parent);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateStampAnnot  (	HPDF_Page           page,
//								HPDF_Rect           rect,
//								HPDF_StampAnnotName name,
//								const char*			text,
//								HPDF_Encoder		encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateProjectionAnnot(HPDF_Page page,
//								HPDF_Rect rect,
//								const char* text,
//								HPDF_Encoder encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateSquareAnnot (HPDF_Page          page,
//							 HPDF_Rect          rect,
//							 const char			*text,
//							 HPDF_Encoder       encoder);
//
//HPDF_EXPORT(HPDF_Annotation)
//HPDF_Page_CreateCircleAnnot (HPDF_Page          page,
//							 HPDF_Rect          rect,
//							 const char			*text,
//							 HPDF_Encoder       encoder);
//
    pub fn HPDF_LinkAnnot_SetHighlightMode  (annot: HPDF_Annotation, 
                                              mode : HPDF_AnnotHighlightMode   )->HPDF_STATUS;

//HPDF_EXPORT(HPDF_STATUS)
//HPDF_LinkAnnot_SetJavaScript(HPDF_Annotation    annot,
//                             HPDF_JavaScript    javascript);
//
    pub fn HPDF_LinkAnnot_SetBorderStyle  (annot   : HPDF_Annotation,
                                            width   : HPDF_REAL,  
                                            dash_on : HPDF_UINT16,      
                                            dash_off: HPDF_UINT16      )->HPDF_STATUS;


//HPDF_EXPORT(HPDF_STATUS)
//HPDF_TextAnnot_SetIcon  (HPDF_Annotation   annot,
//                         HPDF_AnnotIcon    icon);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_TextAnnot_SetOpened  (HPDF_Annotation   annot,
//                          HPDF_BOOL          opened);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annot_SetRGBColor (HPDF_Annotation annot, HPDF_RGBColor color);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annot_SetCMYKColor (HPDF_Annotation annot, HPDF_CMYKColor color);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annot_SetGrayColor (HPDF_Annotation annot, HPDF_REAL color);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annot_SetNoColor (HPDF_Annotation annot);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetTitle (HPDF_Annotation annot, const char* name);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetSubject (HPDF_Annotation annot, const char* name);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetCreationDate (HPDF_Annotation annot, HPDF_Date value);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetTransparency (HPDF_Annotation annot, HPDF_REAL value);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetIntent (HPDF_Annotation  annot, HPDF_AnnotIntent  intent);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetPopup (HPDF_Annotation annot, HPDF_Annotation popup);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetRectDiff (HPDF_Annotation  annot, HPDF_Rect  rect); /* RD entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetCloudEffect (HPDF_Annotation  annot, HPDF_INT cloudIntensity); /* BE entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetInteriorRGBColor (HPDF_Annotation  annot, HPDF_RGBColor color); /* IC with RGB entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetInteriorCMYKColor (HPDF_Annotation  annot, HPDF_CMYKColor color); /* IC with CMYK entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetInteriorGrayColor (HPDF_Annotation  annot, HPDF_REAL color); /* IC with Gray entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_MarkupAnnot_SetInteriorTransparent (HPDF_Annotation  annot); /* IC with No Color entry */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_TextMarkupAnnot_SetQuadPoints ( HPDF_Annotation annot, HPDF_Point lb, HPDF_Point rb, HPDF_Point rt, HPDF_Point lt); /* l-left, r-right, b-bottom, t-top positions */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annot_Set3DView  ( HPDF_MMgr mmgr,
//					 	HPDF_Annotation	annot,
//					 	HPDF_Annotation	annot3d,
//					 	HPDF_Dict			view);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_PopupAnnot_SetOpened  (HPDF_Annotation   annot,
//                            HPDF_BOOL         opened);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_FreeTextAnnot_SetLineEndingStyle (HPDF_Annotation annot, HPDF_LineAnnotEndingStyle startStyle, HPDF_LineAnnotEndingStyle endStyle);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_FreeTextAnnot_Set3PointCalloutLine (HPDF_Annotation annot, HPDF_Point startPoint, HPDF_Point kneePoint, HPDF_Point endPoint); /* Callout line will be in default user space */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_FreeTextAnnot_Set2PointCalloutLine (HPDF_Annotation annot, HPDF_Point startPoint, HPDF_Point endPoint); /* Callout line will be in default user space */
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_FreeTextAnnot_SetDefaultStyle (HPDF_Annotation  annot, const char* style);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_LineAnnot_SetPosition (HPDF_Annotation annot,
//							HPDF_Point startPoint, HPDF_LineAnnotEndingStyle startStyle,
//							HPDF_Point endPoint, HPDF_LineAnnotEndingStyle endStyle);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_LineAnnot_SetLeader (HPDF_Annotation annot, HPDF_INT leaderLen, HPDF_INT leaderExtLen, HPDF_INT leaderOffsetLen);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_LineAnnot_SetCaption (HPDF_Annotation annot, HPDF_BOOL showCaption, HPDF_LineAnnotCapPosition position, HPDF_INT horzOffset, HPDF_INT vertOffset);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Annotation_SetBorderStyle  (HPDF_Annotation  annot,
//                                 HPDF_BSSubtype   subtype,
//                                 HPDF_REAL        width,
//                                 HPDF_UINT16      dash_on,
//                                 HPDF_UINT16      dash_off,
//                                 HPDF_UINT16      dash_phase);
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_ProjectionAnnot_SetExData(HPDF_Annotation annot, HPDF_ExData exdata);

/*--------------------------------------------------------------------------*/
/*----- image data ---------------------------------------------------------*/

    pub fn HPDF_LoadPngImageFromMem  ( pdf:    HPDF_Doc,
                                    buffer: *const HPDF_BYTE,
                                    size:   HPDF_UINT    )->HPDF_Image;

    pub fn HPDF_LoadPngImageFromFile (pdf:         HPDF_Doc   ,
                                   filename:    *const libc::c_char)->HPDF_Image;


    pub fn HPDF_LoadPngImageFromFile2 (pdf:        HPDF_Doc,
                                        filename: *const libc::c_char )->HPDF_Image;


    pub fn HPDF_LoadJpegImageFromFile (pdf:        HPDF_Doc,
                                        filename:  *const libc::c_char)->HPDF_Image;

    pub fn HPDF_LoadJpegImageFromMem  (pdf:    HPDF_Doc  ,
                                      buffer: *const HPDF_BYTE ,
                                      size:   HPDF_UINT)->HPDF_Image;

//HPDF_EXPORT(HPDF_Image)
//HPDF_LoadU3DFromFile (HPDF_Doc      pdf,
//                            const char    *filename);
//
//HPDF_EXPORT(HPDF_Image)
//HPDF_LoadU3DFromMem  (HPDF_Doc      pdf,
//               const HPDF_BYTE     *buffer,
//                     HPDF_UINT      size);

    pub fn HPDF_Image_LoadRaw1BitImageFromMem ( pdf:          HPDF_Doc,          
                                              buf:          *const HPDF_BYTE, 
                                              width:        HPDF_UINT,         
                                              height:       HPDF_UINT,         
                                              line_width:   HPDF_UINT,         
                                              black_is1:    HPDF_BOOL,         
                                              top_is_first: HPDF_BOOL  )->HPDF_Image;


    pub fn HPDF_LoadRawImageFromFile  (pdf:         HPDF_Doc,          
                                    filename:    *const libc::c_char,         
                                    width:       HPDF_UINT,          
                                    height:      HPDF_UINT,          
                                    color_space: HPDF_ColorSpace )->HPDF_Image;


    pub fn HPDF_LoadRawImageFromMem  (pdf:         HPDF_Doc,       
                                   buf:         *const HPDF_BYTE,
                                   width:       HPDF_UINT,    
                                   height:      HPDF_UINT,     
                                   color_space: HPDF_ColorSpace,
                                   bits_per_component: HPDF_UINT)->HPDF_Image;

    pub fn HPDF_Image_AddSMask  (   image : HPDF_Image, 
                                     smask : HPDF_Image  )->HPDF_STATUS;

     pub fn HPDF_Image_GetSize (image: HPDF_Image)->HPDF_Point;


    pub fn HPDF_Image_GetSize2 (image: HPDF_Image, size: *mut HPDF_Point)->HPDF_STATUS;


    pub fn HPDF_Image_GetWidth  (image: HPDF_Image  )->HPDF_UINT;


    pub fn HPDF_Image_GetHeight  (image: HPDF_Image )->HPDF_UINT;


//HPDF_EXPORT(HPDF_UINT)
//HPDF_Image_GetBitsPerComponent (HPDF_Image  image);
//
//
//HPDF_EXPORT(const char*)
//HPDF_Image_GetColorSpace (HPDF_Image  image);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Image_SetColorMask (HPDF_Image   image,
//                         HPDF_UINT    rmin,
//                         HPDF_UINT    rmax,
//                         HPDF_UINT    gmin,
//                         HPDF_UINT    gmax,
//                         HPDF_UINT    bmin,
//                         HPDF_UINT    bmax);
//
//
//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Image_SetMaskImage  (HPDF_Image   image,
//                          HPDF_Image   mask_image);
//


/*--- Compatibility ------------------------------------------------------*/

/* BX --not implemented yet */
/* EX --not implemented yet */

    pub fn HPDF_Page_DrawImage  (page  :   HPDF_Page,   
                              image :   HPDF_Image,   
                              x     :   HPDF_REAL,   
                              y     :   HPDF_REAL,   
                              width :   HPDF_REAL,   
                              height:   HPDF_REAL )->HPDF_STATUS;


    pub fn HPDF_Page_Circle  (page :     HPDF_Page,     
                              x    :     HPDF_REAL,     
                              y    :     HPDF_REAL,     
                              ray  :     HPDF_REAL )->HPDF_STATUS;


    pub fn HPDF_Page_Ellipse  ( page    : HPDF_Page, 
                                 x       : HPDF_REAL, 
                                 y       : HPDF_REAL, 
                                xray     : HPDF_REAL, 
                                yray     : HPDF_REAL  )->HPDF_STATUS;


    pub fn HPDF_Page_Arc  (page     : HPDF_Page,    
                            x        : HPDF_REAL,    
                            y        : HPDF_REAL,    
                            ray      : HPDF_REAL,    
                            ang1     : HPDF_REAL,    
                            ang2     : HPDF_REAL  )->HPDF_STATUS;


   pub fn  HPDF_Page_TextOut  (page:     HPDF_Page,
                                xpos:     HPDF_REAL,
                                ypos:     HPDF_REAL,
                                text:    *const libc::c_char)->HPDF_STATUS;
    

    pub fn HPDF_Page_TextRect  (page   : HPDF_Page,            
                                 left   : HPDF_REAL,            
                                 top    : HPDF_REAL,            
                                 right  : HPDF_REAL,            
                                 bottom : HPDF_REAL,   
                                 text   : *const libc::c_char,
                                 align  : HPDF_TextAlignment,   
                                 len    : *mut HPDF_UINT  );


//HPDF_EXPORT(HPDF_STATUS)
//HPDF_Page_SetSlideShow  (HPDF_Page              page,
//                         HPDF_TransitionStyle   type,
//                         HPDF_REAL              disp_time,
//                         HPDF_REAL              trans_time);
//
//
//HPDF_EXPORT(HPDF_OutputIntent)
//HPDF_ICC_LoadIccFromMem (HPDF_Doc   pdf,
//                        HPDF_MMgr   mmgr,
//                        HPDF_Stream iccdata,
//                        HPDF_Xref   xref,
//                        int         numcomponent);
//
//HPDF_EXPORT(HPDF_OutputIntent)
//HPDF_LoadIccProfileFromFile  (HPDF_Doc  pdf,
//                            const char* icc_file_name,
//                                   int  numcomponent);
//








}










