#![no_std]
use embedded_hal::spi::SpiDevice;
use embedded_hal::digital::OutputPin;

const RH_RF95_REG_00_FIFO: u8 = 0x00;
const RH_RF95_REG_01_OP_MODE: u8 = 0x01;
const RH_RF95_REG_06_FRF_MSB: u8 = 0x06;
const RH_RF95_REG_07_FRF_MID: u8 = 0x07;
const RH_RF95_REG_08_FRF_LSB: u8 = 0x08;
const RH_RF95_REG_09_PA_CONFIG: u8 = 0x09;
const RH_RF95_REG_0A_PA_RAMP: u8 = 0x0A;
const RH_RF95_REG_0B_OCP: u8 = 0x0B;
const RH_RF95_REG_0C_LNA: u8 = 0x0C;
const RH_RF95_REG_0D_FIFO_ADDR_PTR: u8 = 0x0D;
const RH_RF95_REG_0E_FIFO_TX_BASE_ADDR: u8 = 0x0E;
const RH_RF95_REG_0F_FIFO_RX_BASE_ADDR: u8 = 0x0F;
const RH_RF95_REG_10_FIFO_RX_CURRENT_ADDR: u8 = 0x10;
const RH_RF95_REG_11_IRQ_FLAGS_MASK: u8 = 0x11;
const RH_RF95_REG_12_IRQ_FLAGS: u8 = 0x12;
const RH_RF95_REG_13_RX_NB_BYTES: u8 = 0x13;
const RH_RF95_REG_14_RX_HEADER_CNT_VALUE_MSB: u8 = 0x14;
const RH_RF95_REG_15_RX_HEADER_CNT_VALUE_LSB: u8 = 0x15;
const RH_RF95_REG_16_RX_PACKET_CNT_VALUE_MSB: u8 = 0x16;
const RH_RF95_REG_17_RX_PACKET_CNT_VALUE_LSB: u8 = 0x17;
const RH_RF95_REG_18_MODEM_STAT: u8 = 0x18;
const RH_RF95_REG_19_PKT_SNR_VALUE: u8 = 0x19;
const RH_RF95_REG_1A_PKT_RSSI_VALUE: u8 = 0x1A;
const RH_RF95_REG_1B_RSSI_VALUE: u8 = 0x1B;
const RH_RF95_REG_1C_HOP_CHANNEL: u8 = 0x1C;
const RH_RF95_REG_1D_MODEM_CONFIG1: u8 = 0x1D;
const RH_RF95_REG_1E_MODEM_CONFIG2: u8 = 0x1E;
const RH_RF95_REG_1F_SYMB_TIMEOUT_LSB: u8 = 0x1F;
const RH_RF95_REG_20_PREAMBLE_MSB: u8 = 0x20;
const RH_RF95_REG_21_PREAMBLE_LSB: u8 = 0x21;
const RH_RF95_REG_22_PAYLOAD_LENGTH: u8 = 0x22;
const RH_RF95_REG_23_MAX_PAYLOAD_LENGTH: u8 = 0x23;
const RH_RF95_REG_24_HOP_PERIOD: u8 = 0x24;
const RH_RF95_REG_25_FIFO_RX_BYTE_ADDR: u8 = 0x25;
const RH_RF95_REG_26_MODEM_CONFIG3: u8 = 0x26;
const RH_RF95_REG_40_DIO_MAPPING1: u8 = 0x40;
const RH_RF95_REG_41_DIO_MAPPING2: u8 = 0x41;
const RH_RF95_REG_42_VERSION: u8 = 0x42;
const RH_RF95_REG_4B_TCXO: u8 = 0x4B;
const RH_RF95_REG_4D_PA_DAC: u8 = 0x4D;
const RH_RF95_REG_5B_FORMER_TEMP: u8 = 0x5B;
const RH_RF95_REG_61_AGC_REF: u8 = 0x61;
const RH_RF95_REG_62_AGC_THRESH1: u8 = 0x62;
const RH_RF95_REG_63_AGC_THRESH2: u8 = 0x63;
const RH_RF95_REG_64_AGC_THRESH3: u8 = 0x64;
const RH_RF95_DETECTION_OPTIMIZE: u8 = 0x31;
const RH_RF95_DETECTION_THRESHOLD: u8 = 0x37;
const RH_RF95_PA_DAC_DISABLE: u8 = 0x04;
const RH_RF95_PA_DAC_ENABLE: u8 = 0x07;

const SLEEP_MODE: u8 = 0b000;
const STANDBY_MODE: u8 = 0b001;
const FS_TX_MODE: u8 = 0b010;
const TX_MODE: u8 = 0b011;
const FS_RX_MODE: u8 = 0b100;
const RX_MODE: u8 = 0b101;

struct Register<'a> {
    name: &'a str,
    address: u8,
}

impl<'a> Register<'a> {
    fn new(name: &'a str, address: u8) -> Self {
        Self { name, address }
    }
    fn read<SPI: SpiDevice>(&self, spi: &mut SPI) -> u8 {
        todo!()
    }
    fn write<SPI: SpiDevice>(&self, spi: &mut SPI, value: u8) {
        todo!()
    }
}

pub struct RFM95<'a, SPI: SpiDevice, RST: OutputPin> {
    spi: SPI,
    reset: RST, 
    operation_mode: Register<'a>,
    low_frequency_mode: Register<'a>,
    modulation_type: Register<'a>,
    long_range_mode: Register<'a>,
    output_power: Register<'a>,
    max_power: Register<'a>,
    pa_select: Register<'a>,
    pa_dac: Register<'a>,
    dio0_mapping: Register<'a>,
    auto_agc: Register<'a>,
    low_datarate_optimize: Register<'a>,
    lna_boost_hf: Register<'a>,
    auto_ifon: Register<'a>,
    detection_optimize: Register<'a>,
}

impl<'a, SPI, RST> RFM95<'a, SPI, RST>
where
    SPI: SpiDevice,
    RST: OutputPin
{
    pub fn new(spi: SPI, reset: RST) -> Self {
        Self {
            spi,
            reset,
            operation_mode: Register::new("operation_mode", RH_RF95_REG_01_OP_MODE),
            low_frequency_mode: Register::new("low_frequency_mode", RH_RF95_REG_01_OP_MODE),
            modulation_type: Register::new("modulation_type", RH_RF95_REG_01_OP_MODE),
            long_range_mode: Register::new("long_range_mode", RH_RF95_REG_01_OP_MODE),
            output_power: Register::new("output_power", RH_RF95_REG_09_PA_CONFIG),
            max_power: Register::new("max_power", RH_RF95_REG_09_PA_CONFIG),
            pa_select: Register::new("pa_select", RH_RF95_REG_09_PA_CONFIG),
            pa_dac: Register::new("pa_dac", RH_RF95_REG_09_PA_CONFIG),
            dio0_mapping: Register::new("dio0_mapping", RH_RF95_REG_09_PA_CONFIG),
            auto_agc: Register::new("auto_agc", RH_RF95_REG_09_PA_CONFIG),
            low_datarate_optimize: Register::new("low_datarate_optimize", RH_RF95_REG_09_PA_CONFIG),
            lna_boost_hf: Register::new("lna_boost_hf", RH_RF95_REG_09_PA_CONFIG),
            auto_ifon: Register::new("auto_ifon", RH_RF95_REG_09_PA_CONFIG),
            detection_optimize: Register::new("detection_optimize", RH_RF95_REG_09_PA_CONFIG),
        }
    }
}
