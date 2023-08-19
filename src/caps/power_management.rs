use crate::access::Access;
use crate::error::Result;
use nom::sequence::tuple;
use nom::IResult;
use nom::{bits, streaming::take};
use std::fmt::Display;
use std::rc::Rc;

use super::Capability;
use super::Flag;

type IResultCapability<'a> = IResult<&'a [u8], (u8, u8, u8, u8, u8, u8, u8, u8)>;

pub struct PowerManagementCapability {
    _access: Rc<Box<dyn Access>>,
    offset: u8,

    version: u8,
    pme_clock: Flag,
    _immediate_readiness_on_return_to_d0: Flag,
    driver_specific_initialization: Flag,
    aux_current: AuxCurrent,
    d1_support: Flag,
    d2_support: Flag,
    pme_support: PmeSupport,
}

impl PowerManagementCapability {
    pub fn new(access: Rc<Box<dyn Access>>, offset: u8) -> Result<PowerManagementCapability> {
        let mut raw = access.read(offset as u64 + 2, 2)?;
        // TODO check endianness
        raw.reverse();
        let (
            _,
            (
                pme_support,
                d2_support,
                d1_support,
                aux_current,
                driver_specific_initialization,
                immediate_readiness_on_return_to_d0,
                pme_clock,
                version,
            ),
        ) = Self::parse(&raw).unwrap();
        Ok(PowerManagementCapability {
            _access: access,
            offset,
            pme_support: PmeSupport::new(pme_support),
            d2_support: Flag::new("D2", d2_support != 0),
            d1_support: Flag::new("D1", d1_support != 0),
            aux_current: AuxCurrent::new(aux_current),
            driver_specific_initialization: Flag::new("DSI", driver_specific_initialization != 0),
            _immediate_readiness_on_return_to_d0: Flag::new(
                " ",
                immediate_readiness_on_return_to_d0 != 0,
            ),
            pme_clock: Flag::new("PMEClk", pme_clock != 0),
            version,
        })
    }

    fn parse(input: &[u8]) -> IResultCapability {
        bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(tuple((
            take(5usize), // PME_Support
            take(1usize), // D2_Support
            take(1usize), // D1_Support
            take(3usize), // Aux_Current
            take(1usize), // Driver Specific Initialization
            take(1usize), // Immediate_Readiness_on_Return_to_D0
            take(1usize), // PME Clock
            take(3usize), // Version
        )))(input)
    }
}

impl Capability for PowerManagementCapability {
    fn cap_string(&self, verbosity: u8) -> Result<String> {
        let mut text = format!("Power Management version {}\n", self.version);

        if verbosity >= 2 {
            text += &format!(
                "\t\tFlags: {} {} {} {} {} {}\n",
                self.pme_clock,
                self.driver_specific_initialization,
                self.d1_support,
                self.d2_support,
                self.aux_current,
                self.pme_support
            );
        }

        Ok(text.trim().to_string())
    }

    fn offset(&self) -> Result<u64> {
        Ok(self.offset.into())
    }
}

impl Display for PowerManagementCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cap_string(0)?)
    }
}

pub struct AuxCurrent {
    current: u8,
}

impl AuxCurrent {
    pub fn new(current: u8) -> AuxCurrent {
        AuxCurrent { current }
    }
}

impl Display for AuxCurrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuxCurrent={}mA",
            match self.current {
                1 => 55,
                2 => 100,
                3 => 160,
                4 => 220,
                5 => 270,
                6 => 320,
                7 => 375,
                _ => 0,
            }
        )
    }
}

pub struct PmeSupport {
    d0: Flag,
    d1: Flag,
    d2: Flag,
    d3_hot: Flag,
    d3_cold: Flag,
}

impl PmeSupport {
    pub fn new(states: u8) -> PmeSupport {
        PmeSupport {
            d0: Flag::new("D0", states & 0b00001 != 0),
            d1: Flag::new("D1", states & 0b00010 != 0),
            d2: Flag::new("D2", states & 0b00100 != 0),
            d3_hot: Flag::new("D3hot", states & 0b01000 != 0),
            d3_cold: Flag::new("D3cold", states & 0b10000 != 0),
        }
    }
}

impl Display for PmeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PME({},{},{},{},{})",
            self.d0, self.d1, self.d2, self.d3_hot, self.d3_cold
        )
    }
}
