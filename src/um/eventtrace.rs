// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::ULONG64;
use shared::guiddef::GUID;
use shared::minwindef::LPFILETIME;
use shared::ntdef::{ULONG, LONGLONG, ULONGLONG, LPWSTR, WCHAR, LONG, PVOID, USHORT, UCHAR,
                    LARGE_INTEGER};
use um::minwinbase::SYSTEMTIME;
//use um::winnt::ANYSIZE_ARRAY;

STRUCT!{struct TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
}}
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;

STRUCT!{ struct EVENT_DESCRIPTOR {
    Id: USHORT,
    Version: UCHAR,
    Channel: UCHAR,
    Level: UCHAR,
    Opcode: UCHAR,
    Task: USHORT,
    Keyword: ULONGLONG,
}}
pub type PEVENT_DESCRIPTOR = *mut EVENT_DESCRIPTOR;

STRUCT!{ struct EVENT_HEADER_EXTENDED_DATA_ITEM_s {
    Linkage: USHORT,
    Rserved2: USHORT,
}}
pub type PEVENT_HEADER_EXTENDED_DATA_ITEM_s = *mut EVENT_HEADER_EXTENDED_DATA_ITEM_s;

STRUCT!{ struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    Reserved1: USHORT,
    ExtType: USHORT,
    EventHeaderExtendedDataItem_s: EVENT_HEADER_EXTENDED_DATA_ITEM_s,
    DataSize: USHORT,
    DataPtr: ULONGLONG,
}}
pub type PEVENT_HEADER_EXTENDED_DATA_ITEM = *mut EVENT_HEADER_EXTENDED_DATA_ITEM;

STRUCT! { struct EVENT_HEADER_u_s {
    KernelTime: ULONG,
    UserTime: ULONG,
}}

UNION2!{ union EVENT_HEADER_u {
    [ULONG; 2],
    EventHeader_u_s EventHeader_u_s_mut: EVENT_HEADER_u_s,
    ProcessTime ProcessTime_mut: ULONG64,
}}

STRUCT!{ struct EVENT_HEADER { 
    Size: USHORT,
    HeaderType: USHORT,
    Flags: USHORT,
    EventProperty: USHORT,
    ThreadID: ULONG,
    ProcessId: ULONG,
    TimeStamp: LARGE_INTEGER,
    ProviderId: GUID,
    EventDescriptor: EVENT_DESCRIPTOR,
    EventHeader_u: EVENT_HEADER_u,
    ActivityId: GUID,
}}
pub type PEVENT_HEADER = *mut EVENT_HEADER;

STRUCT!{ struct EVENT_RECORD {
    EventHeader: EVENT_HEADER,
    BufferContext: ETW_BUFFER_CONTEXT,
    ExtendedDataCount: USHORT,
    UserDataLength: USHORT,
    ExtendedData: PEVENT_HEADER_EXTENDED_DATA_ITEM,
    UserData: PVOID,
    UserContext: PVOID,
}}
pub type PEVENT_RECORD = *mut EVENT_RECORD;

FN!{stdcall PEVENT_CALLBACK(
    pEvent: PEVENT_TRACE,
) -> ULONG}

FN!{stdcall PEVENT_RECORD_CALLBACK(
    pEventRecord: PEVENT_RECORD,
) -> ()}

FN!{stdcall PEVENT_TRACE_BUFFER_CALLBACK(
   LogFile: PEVENT_TRACE_LOGFILE, 
) -> ULONG}

DEFINE_GUID!{
    EventTraceGuid,
    0x68fdd900, 0x4a3e, 0x11d1, 0x84, 0xf4, 0x00, 0x00, 0xf8, 0x04, 0x64, 0xe3}

DEFINE_GUID!{
    SystemTraceControlGuid,
    0x9e814aad, 0x3204, 0x11d2, 0x9a, 0x82, 0x00, 0x60, 0x08, 0xa8, 0x69, 0x39}

DEFINE_GUID!{ 
    EventTraceConfigGuid,
    0x01853a65, 0x418f, 0x4f36, 0xae, 0xfc, 0xdc, 0x0f, 0x1d, 0x2f, 0xd2, 0x35}

DEFINE_GUID!{
    DefaultTraceSecurityGuid,
    0x0811c1af, 0x7a07, 0x4a06, 0x82, 0xed, 0x86, 0x94, 0x55, 0xcd, 0xf7, 0x13}

pub const PROCESS_TRACE_MODE_REAL_TIME: ULONG = 0x00000100;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: ULONG = 0x00001000;
pub const PROCESS_TRACE_MODE_EVENT_RECORD: ULONG = 0x10000000;

pub const KERNEL_LOGGER_NAMEW: &'static str = "NT Kernel Logger";
pub const GLOBAL_LOGGER_NAMEW: &'static str = "GlobalLogger";
pub const EVENT_LOGGER_NAMEW: &'static str = "EventLog";
pub const DIAG_LOGGER_NAMEW: &'static str = "DiagLog";

pub const KERNEL_LOGGER_NAMEA: &'static str = "NT Kernel Logger";
pub const GLOBAL_LOGGER_NAMEA: &'static str = "GlobalLogger";
pub const EVENT_LOGGER_NAMEA: &'static str = "EventLog";
pub const DIAG_LOGGER_NAMEA: &'static str = "DiagLog";

pub const MAX_MOF_FIELDS: ULONG = 16;

pub type TRACEHANDLE = ULONG64;
pub type PTRACEHANDLE = *mut TRACEHANDLE;

STRUCT!{ struct EVENT_TRACE_HEADER_s {
    HeaderType: UCHAR,
    MarkerFlags: UCHAR,
}}
pub type PEVENT_TRACE_HEADER_s = *mut EVENT_TRACE_HEADER_s;

UNION2!{ union EVENT_TRACE_HEADER_u {
    [USHORT; 1],
    FieldTypeFlags FieldTypeFlags_mut: USHORT,
    EventTraveHeaderSt EventTraveHeaderSt_mut: EVENT_TRACE_HEADER_s,
}}

STRUCT!{ struct EVENT_TRACE_HEADER_s2 {
    Type: UCHAR,
    Level: UCHAR,
    Version: USHORT,
}}
pub type PEVENT_TRACE_HEADER_s2 = *mut EVENT_TRACE_HEADER_s2;

UNION2!{ union EVENT_TRACE_HEADER_u2 {
    [ULONG; 1],
    Version Version_mut: ULONG,
    Class Class_mut: EVENT_TRACE_HEADER_s2,
}}

UNION2!{ union EVENT_TRACE_HEADER_u3 {
    [GUID; 1],
    Guid Guid_mut: GUID,
    GuidPtr GuidPtr_mut: ULONGLONG,
}}

UNION2!{ union EVENT_TRACE_HEADER_u4 {
    [ULONG64; 1],
    EventTraceHeaderSt3 EventTraceHeaderSt3_mut: EVENT_TRACE_HEADER_s3,
    ProcessorTime ProcessorTime_mut: ULONG64, 
    EventTraceHeaderSt4 EventTraceHeaderSt4_mut: EVENT_TRACE_HEADER_s4,
}}

STRUCT!{ struct EVENT_TRACE_HEADER_s3 {
    KernalTime: ULONG,
    UserTime: ULONG,
}}
pub type PEVENT_TRACE_HEADER_s3 = *mut EVENT_TRACE_HEADER_s3;

STRUCT!{ struct EVENT_TRACE_HEADER_s4 {
    ClientContext: ULONG,
    Flags: ULONG,
}}
pub type PEVENT_TRACE_HEADER_s4 = *mut EVENT_TRACE_HEADER_s4;

STRUCT!{ struct EVENT_TRACE_HEADER {
    Size: USHORT,
    un1: EVENT_TRACE_HEADER_u,
    un2: EVENT_TRACE_HEADER_u2,
    ThreadId: ULONG,
    ProcessId: ULONG,
    TimeStamp: LARGE_INTEGER,
    un3: EVENT_TRACE_HEADER_u3,
    un4: EVENT_TRACE_HEADER_u4,
}}
pub type PEVENT_TRACE_HEADER = *mut EVENT_TRACE_HEADER;

STRUCT!{ struct ETW_BUFFER_CONTEXT_u_s {
    ProcessorNumber: UCHAR,
    Alignment: UCHAR,
}}
pub type PETW_BUFFER_CONTEXT_u_s = *mut ETW_BUFFER_CONTEXT_u_s;

UNION2!{ union ETW_BUFFER_CONTEXT_u {
    [UCHAR; 2],
    EtwBufferContext_u_s EtwBufferContext_u_s_mut: ETW_BUFFER_CONTEXT_u_s,
    ProcessorIndex ProcessorIndex_mut: ETW_BUFFER_CONTEXT,
}}

STRUCT!{ struct ETW_BUFFER_CONTEXT {
    EtwBufferContext_u: ETW_BUFFER_CONTEXT_u,
    LoggerId: USHORT,
}}
pub type PETW_BUFFER_CONTEXT = *mut ETW_BUFFER_CONTEXT;

UNION2!{ union EVENT_TRACE_u {
    [ULONG; 1],
    ClientContext ClientContext_mut: ULONG,
    BufferContext BufferContext_mut: ETW_BUFFER_CONTEXT,
}}

STRUCT!{ struct EVENT_TRACE {
    Header: EVENT_TRACE_HEADER,
    InstanceId: ULONG,
    ParentInstanceId: ULONG,
    ParentGuid: GUID,
    MofData: PVOID,
    MofLength: ULONG,
    EventTrace_u: EVENT_TRACE_u,
}}
pub type PEVENT_TRACE = *mut EVENT_TRACE;

UNION2!{ union EVENT_TRACE_LOGFILE_u {
    [ULONG; 1],
    LogFileMode LogFileMode_mut: ULONG,
    ProcessTraceMode ProcessTraceMode_mut: ULONG, 
}}

UNION2!{ union EVENT_TRACE_LOGFILE_u2 {
    [u32; 1] [u64; 1],
    EventCallback EventCallback_mut: PEVENT_CALLBACK,
    EventRecordCallback EventRecordCallback_mut: PEVENT_RECORD_CALLBACK,
}}

UNION2!{ union TRACE_LOGFILE_HEADER_u {
    [ULONG; 1],
    Version Version_mut: ULONG,
    VersionDetail VersionDetail_mut: VersionDetail,
}}

STRUCT!{struct VersionDetail {
    MajorVersion: UCHAR,
    MinorVersion: UCHAR,
    SubVersion: UCHAR,
    SubMinorVersion: UCHAR,
}}

STRUCT!{struct TRACE_LOGFILE_HEADER_s {
    StartBuffers: ULONG,
    PointerSize: ULONG,
    EventsLost: ULONG,
    CpuSpeedInMHz: ULONG,
}}

UNION2!{ union TRACE_LOGFILE_HEADER_u2 {
    [GUID; 1],
    LogInstanceGuid LogInstanceGuid_mut: GUID,
    TraceLogFileHeader_s TraceLogFileHeader_s_mut: TRACE_LOGFILE_HEADER_s, 
}}

STRUCT!{ struct TRACE_LOGFILE_HEADER {
    BufferSize: ULONG,
    TraceLogFileHeader_u: TRACE_LOGFILE_HEADER_u,
    ProviderVersion: ULONG,
    NumberOfProcessors: ULONG,
    EndTime: LARGE_INTEGER,
    timeResolution: ULONG,
    MaximumFileSize: ULONG,
    LogFileMode: ULONG,
    BuffersWritten: ULONG,
    TraceLogFileHeader_u2: TRACE_LOGFILE_HEADER_u2,
    LoggerName: LPWSTR,
    LogFileName: LPWSTR,
    TimeZone: TIME_ZONE_INFORMATION,
    BootTime: LARGE_INTEGER,
    PerfFreq: LARGE_INTEGER,
    StartTime: LARGE_INTEGER,
    ReservedFlags: ULONG,
    BuffersLost: ULONG,
}}
pub type PTRACE_LOGFILE_HEADER = *mut TRACE_LOGFILE_HEADER;

STRUCT!{struct EVENT_TRACE_LOGFILE {
    LogFileName: LPWSTR,
    LoggerName: LPWSTR,
    CurrentTime: LONGLONG,
    BuffersRead: ULONG,
    EventTraceLogFile_u: EVENT_TRACE_LOGFILE_u,
    CurrentEvent: EVENT_TRACE,
    LogFileHeader: TRACE_LOGFILE_HEADER,
    BufferCallback: PEVENT_TRACE_BUFFER_CALLBACK,
    BufferSize: ULONG,
    Filled: ULONG,
    EventsLost: ULONG,
    EventTraceLogFile_u2: EVENT_TRACE_LOGFILE_u2,
    IsKernalTrace: ULONG,
    Context: PVOID,
}}
pub type PEVENT_TRACE_LOGFILE = *mut EVENT_TRACE_LOGFILE;

ENUM!{enum TDH_CONTEXT_TYPE {
    TDH_CONTEXT_WPP_TMFFILE = 0,
    TDH_CONTEXT_WPP_TMFSEARCHPATH = 1,
    TDH_CONTEXT_WPP_GMT = 2,
    TDH_CONTEXT_POINTERSIZE = 3,
    TDH_CONTEXT_PDB_PATH = 4,
    TDH_CONTEXT_MAXIMUM = 5,
}}

STRUCT!{ struct TDH_CONTEXT {
    ParameterValue: ULONGLONG,
    ParameterType: TDH_CONTEXT_TYPE,
    ParameterSize: ULONG,
}}
pub type PTDH_CONTEXT = *mut TDH_CONTEXT;

ENUM!{ enum PROPERTY_FLAGS { 
  PropertyStruct            = 0x1,
  PropertyParamLength       = 0x2,
  PropertyParamCount        = 0x4,
  PropertyWBEMXmlFragment   = 0x8,
  PropertyParamFixedLength  = 0x10,
  PropertyParamFixedCount   = 0x20,
  PropertyHasTags           = 0x40,
  PropertyHasCustomSchema   = 0x80,
}}

STRUCT!{struct EVENT_PROPERTY_INFO_nonStructType {
   InType: USHORT,
   OutType: USHORT,
   MapNameOffset: ULONG,
}}
pub type PEVENT_PROPERTY_INFO_nonStructType = *mut EVENT_PROPERTY_INFO_nonStructType;

STRUCT!{struct EVENT_PROPERTY_INFO_StructType {
   StructStartIndex: USHORT,
   NumOfStructMembers: USHORT,
   padding: ULONG,
}}
pub type PEVENT_PROPERTY_INFO_StructType = *mut EVENT_PROPERTY_INFO_StructType;

STRUCT!{struct EVENT_PROPERTY_INFO_CustomSchemaType {
      padding2: USHORT,
      OutType: USHORT,
      CustomSchemaOffset: ULONG,
}}
pub type PEVENT_PROPERTY_INFO_CustomSchemaType = *mut EVENT_PROPERTY_INFO_CustomSchemaType;

UNION2!{union EVENT_PROPERTY_INFO_u1 {
    [ULONG; 2],
    nonStructType nonStructType_mut: EVENT_PROPERTY_INFO_nonStructType,
    StructType StructType_mut: EVENT_PROPERTY_INFO_StructType,
    CustomSchemaType CustomSchemaType_mut: EVENT_PROPERTY_INFO_CustomSchemaType,
}}

UNION2!{union EVENT_PROPERTY_INFO_u2 {
    [USHORT; 1],
    count count_mut: USHORT,
    countPropertyIndex countPropertyIndex_mut: USHORT,
}}

UNION2!{union EVENT_PROPERTY_INFO_u3 {
    [USHORT; 1],
    length length_mut: USHORT,
    lengthPropertyIndex lengthPropertyIndex_mut: USHORT,
}}

UNION2!{union EVENT_PROPERTY_INFO_u4 {
    [ULONG; 1],
    Tags Tags_mut: ULONG,
    Reserved Reserved_mut: ULONG,
}}

STRUCT!{struct EVENT_PROPERTY_INFO {
    Flags: PROPERTY_FLAGS,
    NameOffset: ULONG,
    EventPropertyInfo_u1: EVENT_PROPERTY_INFO_u1,
    EventPropertyInfo_u2: EVENT_PROPERTY_INFO_u2,
    EventPropertyInfo_u3: EVENT_PROPERTY_INFO_u3,
    EventPropertyInfo_u4: EVENT_PROPERTY_INFO_u4,
}}
pub type PEVENT_PROPERTY_INFO = *mut EVENT_PROPERTY_INFO;

STRUCT!{ struct TRACE_EVENT_INFO_u_s {
     Reserved: ULONG,
     Tags: ULONG,
}}
pub type PTRACE_EVENT_INFO_u_s = *mut TRACE_EVENT_INFO_u_s;

UNION2!{ union TRACE_EVENT_INFO_u {
    [ULONG; 1],
    Flags Flags_mut: TEMPLATE_FLAGS,
    TraceEventInfo_u_s TraceEventInfo_u_s_mut: TRACE_EVENT_INFO_u_s,
}}

ENUM!{enum TEMPLATE_FLAGS { 
  TEMPLATE_EVENT_DATA  = 1,
  TEMPLATE_USER_DATA   = 2,
}}

ENUM!{enum DECODING_SOURCE { 
  DecodingSourceXMLFile  = 0,
  DecodingSourceWbem     = 1,
  DecodingSourceWPP      = 2,
  DecodingSourceTlg      = 3,
}}

STRUCT!{ struct TRACE_EVENT_INFO {
    ProviderGuid: GUID,
    EventGuid: GUID,
    EventDescriptor: EVENT_DESCRIPTOR,
    DecodingSource: DECODING_SOURCE,
    ProviderNameOffset: ULONG,
    LevelNameOffset: ULONG,
    ChannelNameOffset: ULONG,
    KeywordsNameOffset: ULONG,
    TaskNameOffset: ULONG,
    OpcodeNameOffset: ULONG,
    EventMessageOffset: ULONG,
    ProviderMessageOffset: ULONG,
    BinaryXMLOffset: ULONG,
    BinaryXMLSize: ULONG,
    ActivityIDNameOffset: ULONG,
    RelatedActivityIDNameOffset: ULONG,
    PropertyCount: ULONG,
    TopLevelPropertyCount: ULONG, 
    TraceEventInfo_u: TRACE_EVENT_INFO_u, 
    // TODO: ANYSIZE_ARRAY
    EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 10],
}}
pub type PTRACE_EVENT_INFO = *mut TRACE_EVENT_INFO;

ENUM!{enum MAP_FLAGS {
    EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP    = 1,
    EVENTMAP_INFO_FLAG_MANIFEST_BITMAP      = 2,
    EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP  = 4,
    EVENTMAP_INFO_FLAG_WBEM_VALUEMAP        = 8,
    EVENTMAP_INFO_FLAG_WBEM_BITMAP          = 16,
    EVENTMAP_INFO_FLAG_WBEM_FLAG            = 32,
    EVENTMAP_INFO_FLAG_WBEM_NO_MAP          = 64
}}

ENUM!{enum MAP_VALUETYPE {
    EVENTMAP_ENTRY_VALUETYPE_ULONG   = 0,
    EVENTMAP_ENTRY_VALUETYPE_STRING  = 1
}}

UNION2!{union EVENT_MAP_INFO_u {
    [ULONG; 1],
    MapEntryValueType MapEntryValueType_mut: MAP_VALUETYPE,
    FormatStringOffset FormatStringOffset_mut: ULONG,
}}

UNION2!{union EVENT_MAP_ENTRY_u {
    [ULONG; 1],
    Value Value_mut: ULONG,
    InputOffset InputOffset_mut: ULONG,
}}

STRUCT!{struct EVENT_MAP_ENTRY {
    OutputOffset: ULONG,
    EventMapEntry_u: EVENT_MAP_ENTRY_u,
}}

STRUCT!{struct EVENT_MAP_INFO {
    NameOffset: ULONG,
    Flag: MAP_FLAG,
    EntryCount: ULONG,
    EventMapInfo_u: EVENT_MAP_INFO_u,
    // TODO: ANYSIZE_ARRAY
    MapEntryArray: [EVENT_MAP_ENTRY; 10],
}}

STRUCT!{ struct PROPERTY_DATA_DESCRIPTOR{
    PropertyName: ULONGLONG,
    ArrayIndex: ULONG,
    Reserved: ULONG,
}}
pub type PPROPERTY_DATA_DESCRIPTOR = *mut PROPERTY_DATA_DESCRIPTOR;

ENUM!{enum TDH_IN_TYPE {
    TDH_INTYPE_NULL,
    TDH_INTYPE_UNICODESTRING,
    TDH_INTYPE_ANSISTRING,
    TDH_INTYPE_INT8,
    TDH_INTYPE_UINT8,
    TDH_INTYPE_INT16,
    TDH_INTYPE_UINT16,
    TDH_INTYPE_INT32,
    TDH_INTYPE_UINT32,
    TDH_INTYPE_INT64,
    TDH_INTYPE_UINT64,
    TDH_INTYPE_FLOAT,
    TDH_INTYPE_DOUBLE,
    TDH_INTYPE_BOOLEAN,
    TDH_INTYPE_BINARY,
    TDH_INTYPE_GUID,
    TDH_INTYPE_POINTER,
    TDH_INTYPE_FILETIME,
    TDH_INTYPE_SYSTEMTIME,
    TDH_INTYPE_SID,
    TDH_INTYPE_HEXINT32,
    TDH_INTYPE_HEXINT64,                    // End of winmeta intypes.
    TDH_INTYPE_COUNTEDSTRING = 300,         // Start of TDH intypes for WBEM.
    TDH_INTYPE_COUNTEDANSISTRING,
    TDH_INTYPE_REVERSEDCOUNTEDSTRING,
    TDH_INTYPE_REVERSEDCOUNTEDANSISTRING,
    TDH_INTYPE_NONNULLTERMINATEDSTRING,
    TDH_INTYPE_NONNULLTERMINATEDANSISTRING,
    TDH_INTYPE_UNICODECHAR,
    TDH_INTYPE_ANSICHAR,
    TDH_INTYPE_SIZET,
    TDH_INTYPE_HEXDUMP,
    TDH_INTYPE_WBEMSID
}}

ENUM!{enum TDH_OUT_TYPE {
    TDH_OUTTYPE_NULL,
    TDH_OUTTYPE_STRING,
    TDH_OUTTYPE_DATETIME,
    TDH_OUTTYPE_BYTE,
    TDH_OUTTYPE_UNSIGNEDBYTE,
    TDH_OUTTYPE_SHORT,
    TDH_OUTTYPE_UNSIGNEDSHORT,
    TDH_OUTTYPE_INT,
    TDH_OUTTYPE_UNSIGNEDINT,
    TDH_OUTTYPE_LONG,
    TDH_OUTTYPE_UNSIGNEDLONG,
    TDH_OUTTYPE_FLOAT,
    TDH_OUTTYPE_DOUBLE,
    TDH_OUTTYPE_BOOLEAN,
    TDH_OUTTYPE_GUID,
    TDH_OUTTYPE_HEXBINARY,
    TDH_OUTTYPE_HEXINT8,
    TDH_OUTTYPE_HEXINT16,
    TDH_OUTTYPE_HEXINT32,
    TDH_OUTTYPE_HEXINT64,
    TDH_OUTTYPE_PID,
    TDH_OUTTYPE_TID,
    TDH_OUTTYPE_PORT,
    TDH_OUTTYPE_IPV4,
    TDH_OUTTYPE_IPV6,
    TDH_OUTTYPE_SOCKETADDRESS,
    TDH_OUTTYPE_CIMDATETIME,
    TDH_OUTTYPE_ETWTIME,
    TDH_OUTTYPE_XML,
    TDH_OUTTYPE_ERRORCODE,
    TDH_OUTTYPE_WIN32ERROR,
    TDH_OUTTYPE_NTSTATUS,
    TDH_OUTTYPE_HRESULT,             // End of winmeta outtypes.
    TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME, //Culture neutral datetime string.
    TDH_OUTTYPE_REDUCEDSTRING = 300, // Start of TDH outtypes for WBEM.
    TDH_OUTTYPE_NOPRINT
}}

extern "system" {
    pub fn OpenTraceW(Logfile: PEVENT_TRACE_LOGFILE) -> TRACEHANDLE;

    pub fn CloseTrace(TraceHandle: TRACEHANDLE) -> ULONG;

    pub fn ProcessTrace(HandleArray: PTRACEHANDLE,
                        HandleCount: ULONG,
                        StartTime: LPFILETIME,
                        EndTime: LPFILETIME)
                        -> ULONG;

    pub fn TdhGetEventInformation(pEvent: PEVENT_RECORD,
                                  TdhContextCount: ULONG,
                                  pTdhContext: PTDH_CONTEXT,
                                  pBuffer: PTRACE_EVENT_INFO,
                                  pBufferSize: &ULONG)
                                  -> ULONG;

    pub fn TdhGetPropertySize(pEvent: PEVENT_RECORD,
                            TdhContextCount: ULONG,
                            pTdhContext: PTDH_CONTEXT,
                            PropertyDataCount: ULONG,
                            pPropertyData: PPROPERTY_DATA_DESCRIPTOR,
                            pPropertySize: &ULONG
    ) -> ULONG;

    pub fn TdhGetProperty(pEvent: PEVENT_RECORD,
                            TdhContextCount: ULONG,
                            pTdhContext: PTDH_CONTEXT,
                            PropertyDataCount: ULONG,
                            pPropertyData: PPROPERTY_DATA_DESCRIPTOR,
                            BufferSize: ULONG,
                            pBuffer: &BYTE
    ) -> ULONG;

    pub fn TdhGetEventMapInformation(pEvent: PEVENT_RECORD,
        pMapName: LPWSTR,
        pBuffer: PEVENT_MAP_INFO,
        pBufferSize: &ULONG
    ) -> ULONG;
}
