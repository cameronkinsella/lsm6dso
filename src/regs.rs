use core::convert::TryFrom;

#[derive(Debug)]
pub enum RegisterError {
    ConversionError,
}

// Device registers
#[derive(Debug, Clone, Copy)]
pub enum Register {
    FuncCfgAccess = 0x01,

    PinCtrl = 0x02,

    FifoCtrl1 = 0x07,
    FifoCtrl2 = 0x08,
    FifoCtrl3 = 0x09,
    FifoCtrl4 = 0x0A,

    CounterBdrReg1 = 0x0B,
    CounterBdrReg2 = 0x0C,

    Int1Ctrl = 0x0D,
    Int2Ctrl = 0x0E,

    WhoAmI = 0x0F,

    Ctrl1XL = 0x10,
    Ctrl2G = 0x11,
    Ctrl3C = 0x12,
    Ctrl4C = 0x13,
    Ctrl5C = 0x14,
    Ctrl6C = 0x15,
    Ctrl7G = 0x16,
    Ctrl8Xl = 0x17,
    Ctrl9Xl = 0x18,
    Ctrl10C = 0x19,

    AllIntSrc = 0x1A,
    WakeUpSrc = 0x1B,
    TapSrc = 0x1C,
    D6dSrc = 0x1D,

    StatusReg = 0x1E,

    OutTempL = 0x20,
    OutTempH = 0x21,

    OutXLG = 0x22,
    OutXHG = 0x23,
    OutYLG = 0x24,
    OutYHG = 0x25,
    OutZLG = 0x26,
    OutZHG = 0x27,

    OutXLA = 0x28,
    OutXHA = 0x29,
    OutYLA = 0x2A,
    OutYHA = 0x2B,
    OutZLA = 0x2C,
    OutZHA = 0x2D,

    EmbFuncStatusMainpage = 0x35,
    FsmStatusAMainpage = 0x36,
    FsmStatusBMainpage = 0x37,
    StatusMasterMainpage = 0x39,

    FifoStatus1 = 0x3A,
    FifoStatus2 = 0x3B,

    Timestamp0 = 0x40,
    Timestamp1 = 0x41,
    Timestamp2 = 0x42,
    Timestamp3 = 0x43,

    TapCfg0 = 0x56,
    TapCfg1 = 0x57,
    TapCfg2 = 0x58,
    TapThs6d = 0x59,

    IntDur2 = 0x5A,

    WakeUpThs = 0x5B,
    WakeUpDur = 0x5C,

    FreeFall = 0x5D,

    Md1Cfg = 0x5E,
    Md2Cfg = 0x5F,

    I3cBusAvb = 0x62,

    InternalFreqFine = 0x63,

    IntOis = 0x6F,
    Ctrl1Ois = 0x70,
    Ctrl2Ois = 0x71,
    Ctrl3Ois = 0x72,

    XOfsUsr = 0x73,
    YOfsUsr = 0x74,
    ZOfsUsr = 0x75,

    FifoDataOutTag = 0x78,
    FifoDataOutXL = 0x79,
    FifoDataOutXH = 0x7A,
    FifoDataOutYL = 0x7B,
    FifoDataOutYH = 0x7C,
    FifoDataOutZL = 0x7D,
    FifoDataOutZH = 0x7E,
}

impl From<Register> for u8 {
    fn from(r: Register) -> u8 {
        r as u8
    }
}

pub trait RegisterOption {
    fn value(&self) -> u8;
    fn mask() -> u8;
    fn bit_offset() -> u8;
}

// -------------------------------------------------------------------------------------------------
// --- CTRL1_XL ------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum AccelerometerOutput {
    PowerDown = 0b0000,
    /// Low power only. Is 12.5 Hz in high performance mode.
    Rate1_6 = 0b1011,
    Rate12_5 = 0b0001,
    Rate26 = 0b0010,
    Rate52 = 0b0011,
    Rate104 = 0b0100,
    Rate208 = 0b0101,
    Rate416 = 0b0110,
    Rate833 = 0b0111,
    Rate1_66k = 0b1000,
    Rate3_33k = 0b1001,
    Rate6_66k = 0b1010,
}

impl RegisterOption for AccelerometerOutput {
    fn value(&self) -> u8 {
        *self as u8
    }
    fn mask() -> u8 {
        0xF
    }
    fn bit_offset() -> u8 {
        4
    }
}

/// Accelerometer full-scale selection
#[derive(Debug, Clone, Copy)]
pub enum AccelerometerScale {
    G02 = 0b00,
    /// Old full-scale mode only. Is 2g with new full-scale mode.
    G16 = 0b01,
    G04 = 0b10,
    G08 = 0b11,
}

impl RegisterOption for AccelerometerScale {
    fn value(&self) -> u8 {
        *self as u8
    }
    fn mask() -> u8 {
        0x03
    }
    fn bit_offset() -> u8 {
        2
    }
}

impl AccelerometerScale {
    pub fn scale(&self) -> f32 {
        match *self {
            Self::G02 => 0.061,
            Self::G04 => 0.122,
            Self::G08 => 0.244,
            Self::G16 => 0.488,
        }
    }
}

impl TryFrom<u8> for AccelerometerScale {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = (value >> Self::bit_offset()) & Self::mask();
        match value {
            0b00 => Ok(Self::G02),
            0b01 => Ok(Self::G16),
            0b10 => Ok(Self::G04),
            0b11 => Ok(Self::G08),
            _ => Err(RegisterError::ConversionError),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// --- CTRL2_G -------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum GyroscopeOutput {
    PowerDown = 0b0000,
    Rate12_5 = 0b0001,
    Rate26 = 0b0010,
    Rate52 = 0b0011,
    Rate104 = 0b0100,
    Rate208 = 0b0101,
    Rate416 = 0b0110,
    Rate833 = 0b0111,
    Rate1_66k = 0b1000,
    Rate3_33k = 0b1001,
    Rate6_66k = 0b1010,
}

impl RegisterOption for GyroscopeOutput {
    fn value(&self) -> u8 {
        *self as u8
    }
    fn mask() -> u8 {
        0xF
    }
    fn bit_offset() -> u8 {
        4
    }
}

/// Bandwidth of the accelerometer LPF2 digital low pass filter
#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    OdrDiv4 = 0b000,
    OdrDiv10 = 0b001,
    OdrDiv20 = 0b010,
    OdrDiv45 = 0b011,
    OdrDiv100 = 0b100,
    OdrDiv200 = 0b101,
    OdrDiv400 = 0b110,
    OdrDiv800 = 0b111,
}
impl RegisterOption for Bandwidth {
    fn value(&self) -> u8 {
        *self as u8
    }
    fn mask() -> u8 {
        0b111
    }
    fn bit_offset() -> u8 {
        5
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GyroscopeFullScale {
    Dps125 = 0b001,
    Dps250 = 0b000,
    Dps500 = 0b010,
    Dps1000 = 0b100,
    Dps2000 = 0b110,
}

impl RegisterOption for GyroscopeFullScale {
    fn value(&self) -> u8 {
        *self as u8
    }
    fn mask() -> u8 {
        0b111
    }
    fn bit_offset() -> u8 {
        1
    }
}

impl GyroscopeFullScale {
    pub fn scale(&self) -> f32 {
        match *self {
            Self::Dps125 => 4.375,
            Self::Dps250 => 8.75,
            Self::Dps500 => 17.50,
            Self::Dps1000 => 35.0,
            Self::Dps2000 => 70.0,
        }
    }
}

impl TryFrom<u8> for GyroscopeFullScale {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = (value >> Self::bit_offset()) & Self::mask();
        match value {
            0b001 => Ok(GyroscopeFullScale::Dps125),
            0b000 => Ok(GyroscopeFullScale::Dps250),
            0b010 => Ok(GyroscopeFullScale::Dps500),
            0b100 => Ok(GyroscopeFullScale::Dps1000),
            0b110 => Ok(GyroscopeFullScale::Dps2000),
            _ => Err(RegisterError::ConversionError),
        }
    }
}

/// Bit fields for CTRL3_C
#[allow(unused)]
pub enum Ctrl3C {
    Boot = 7,
    BlockDataUpdate = 6,
    InterruptActivationLevel = 5,
    InterruptPadOutput = 4,
    SpiSerialInterfaceMode = 3,
    AutoIncrement = 2,
    SoftwareReset = 0,
}

/// Bit fields for CTRL6_C
#[allow(unused)]
pub enum Ctrl6C {
    GyroEdgeTrigger = 7,
    GyroLevelTrigger = 6,
    GyroLevelLatched = 5,
    AccelHighPerformanceMode = 4,
    AccelOffsetWeight = 3,
    GyroBandwidth2 = 2,
    GyroBandwidth1 = 1,
    GyroBandwidth0 = 0,
}

/// Bit fields for CTRL6_G
#[allow(unused)]
pub enum Ctrl7G {
    GyroHighPerformanceMode = 7,
    GyroHighPassFilter = 6,
    GyroHighPassCutoff1 = 5,
    GyroHighPassCutoff0 = 4,
    EnableOISChainSPI2 = 2,
    EnableAccelOffset = 1,
    EnableOISChainPrimary = 0,
}
