use Module;
use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gain {
    LV,
    HV,
}

impl ::std::convert::Into<String> for Gain {
    fn into(self) -> String {
        let s = match self {
            Gain::LV => "LV",
            Gain::HV => "HV",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for Gain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LV" => Ok(Gain::LV),
            "HV" => Ok(Gain::HV),
            gain => Err(format!("Unknow gain '{}'", gain)),
        }
    }
}

impl ::std::fmt::Display for Gain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Gain::LV => "LV",
            &Gain::HV => "HV",
        };

        write!(f, "{}", display)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    IN1,
    IN2,
}

impl ::std::convert::Into<String> for Source {
    fn into(self) -> String {
        let s = match self {
            Source::IN1 => "SOUR1",
            Source::IN2 => "SOUR2",
        };

        String::from(s)
    }
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::IN1 => "IN 1",
            &Source::IN2 => "IN 2",
        };

        write!(f, "{}", display)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Decimation {
    DEC_1,
    DEC_8,
    DEC_64,
    DEC_1024,
    DEC_8192,
    DEC_65536,
}

impl ::std::convert::Into<String> for Decimation {
    fn into(self) -> String {
        let s = match self {
            Decimation::DEC_1 => "1",
            Decimation::DEC_8 => "8",
            Decimation::DEC_64 => "64",
            Decimation::DEC_1024 => "1024",
            Decimation::DEC_8192 => "8192",
            Decimation::DEC_65536 => "65536",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for Decimation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Decimation::DEC_1),
            "8" => Ok(Decimation::DEC_8),
            "64" => Ok(Decimation::DEC_64),
            "1024" => Ok(Decimation::DEC_1024),
            "8192" => Ok(Decimation::DEC_8192),
            "65536" => Ok(Decimation::DEC_65536),
            decimation => Err(format!("Unknow decimation '{}'", decimation)),
        }
    }
}

impl ::std::convert::Into<SamplingRate> for Decimation {
    fn into(self) -> SamplingRate {
        match self {
            Decimation::DEC_1 => SamplingRate::RATE_125MHz,
            Decimation::DEC_8 => SamplingRate::RATE_15_6MHz,
            Decimation::DEC_64 => SamplingRate::RATE_1_9MHz,
            Decimation::DEC_1024 => SamplingRate::RATE_103_8kHz,
            Decimation::DEC_8192 => SamplingRate::RATE_15_2kHz,
            Decimation::DEC_65536 => SamplingRate::RATE_1_9kHz,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SamplingRate {
    RATE_125MHz,
    RATE_15_6MHz,
    RATE_1_9MHz,
    RATE_103_8kHz,
    RATE_15_2kHz,
    RATE_1_9kHz,
}

impl SamplingRate {
    pub fn get_buffer_duration(self) -> ::std::time::Duration {
        let (s, ns) = match self {
            SamplingRate::RATE_125MHz => (0, 131_072),
            SamplingRate::RATE_15_6MHz => (0, 1_049_000),
            SamplingRate::RATE_1_9MHz => (0, 8_389_000),
            SamplingRate::RATE_103_8kHz => (0, 134_218_000),
            SamplingRate::RATE_15_2kHz => (1, 740_000_000),
            SamplingRate::RATE_1_9kHz => (8, 590_000_000),
        };

        ::std::time::Duration::new(s, ns)
    }
}

impl ::std::fmt::Display for SamplingRate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &SamplingRate::RATE_125MHz => "125 MHz",
            &SamplingRate::RATE_15_6MHz => "15.6 MHz",
            &SamplingRate::RATE_1_9MHz => "1.9 MHz",
            &SamplingRate::RATE_103_8kHz => "103.8 kHz",
            &SamplingRate::RATE_15_2kHz => "15.2 kHz",
            &SamplingRate::RATE_1_9kHz => "1.9 kHz",
        };

        write!(f, "{}", display)
    }
}

impl ::std::convert::Into<Decimation> for SamplingRate {
    fn into(self) -> Decimation {
        match self {
            SamplingRate::RATE_125MHz => Decimation::DEC_1,
            SamplingRate::RATE_15_6MHz => Decimation::DEC_8,
            SamplingRate::RATE_1_9MHz => Decimation::DEC_64,
            SamplingRate::RATE_103_8kHz => Decimation::DEC_1024,
            SamplingRate::RATE_15_2kHz => Decimation::DEC_8192,
            SamplingRate::RATE_1_9kHz => Decimation::DEC_65536,
        }
    }
}

impl ::std::convert::Into<String> for SamplingRate {
    fn into(self) -> String {
        let s = match self {
            SamplingRate::RATE_125MHz => "125MHz",
            SamplingRate::RATE_15_6MHz => "15_6MHz",
            SamplingRate::RATE_1_9MHz => "1_9MHz",
            SamplingRate::RATE_103_8kHz => "103_8kHz",
            SamplingRate::RATE_15_2kHz => "15_2kHz",
            SamplingRate::RATE_1_9kHz => "1_9kHz",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for SamplingRate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "125000000 Hz" => Ok(SamplingRate::RATE_125MHz),
            "15600000 Hz" => Ok(SamplingRate::RATE_15_6MHz),
            "1900000 Hz" => Ok(SamplingRate::RATE_1_9MHz),
            "103800 Hz" => Ok(SamplingRate::RATE_103_8kHz),
            "15200 Hz" => Ok(SamplingRate::RATE_15_2kHz),
            "1900 Hz" => Ok(SamplingRate::RATE_1_9kHz),
            rate => Err(format!("Unknow sampling rate {}", rate)),
        }
    }
}

#[derive(Clone)]
pub struct Acquire {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for Acquire {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl Acquire {
    pub fn new(socket: ::std::cell::RefCell<Socket>) -> Self {
        Acquire {
            socket,
        }
    }

    /**
     * Starts acquisition.
     */
    pub fn start(&mut self) {
        self.send("ACQ:START");
    }

    /**
     * Stops acquisition.
     */
    pub fn stop(&mut self) {
        self.send("ACQ:STOP");
    }

    /**
     * Stops acquisition and sets all parameters to default values.
     */
    pub fn reset(&self) {
        self.send("ACQ:RST");
    }

    /**
     * Set decimation factor.
     */
    pub fn set_decimation(&self, decimation: Decimation) {
        self.send(format!("ACQ:DEC {}", Into::<String>::into(decimation)));
    }

    /**
     * Get decimation factor.
     */
    pub fn get_decimation(&self) -> Result<Decimation, String> {
        self.send("ACQ:DEC?");

        self.receive()
            .parse()
    }

    /**
     * Get sampling rate.
     *
     * # Panics
     *
     * Calling this command makes buffer overflow.
     * See https://github.com/RedPitaya/RedPitaya/pull/110
     */
    pub fn get_sampling_rate(&self) -> Result<SamplingRate, String> {
        self.send("ACQ:SRAT?");

        self.receive()
            .parse()
    }

    /**
     * Enable averaging.
     */
    pub fn enable_average(&self) {
        self.send("ACQ:AVG ON");
    }

    /**
     * Disable averaging.
     */
    pub fn disable_average(&self) {
        self.send("ACQ:AVG OFF");
    }

    /**
     * Get averaging status.
     */
    pub fn is_average_enabled(&self) -> bool {
        self.send("ACQ:AVG?");

        let message = self.receive();

        match message.as_str() {
            "ON" => true,
            _ => false,
        }
    }

    /**
     * Set gain settings to HIGH or LOW.
     *
     * This gain is referring to jumper settings on Red Pitaya fast analog inputs.
     */
    pub fn set_gain(&self, source: Source, gain: Gain) {
        self.send(format!("ACQ:{}:GAIN {}", Into::<String>::into(source), Into::<String>::into(gain)));
    }

    /**
     * Get gain settings to HIGH or LOW.
     */
    pub fn get_gain(&self, source: Source) -> Result<Gain, String> {
        self.send(format!("ACQ:{}:GAIN?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sampling_rate_get_buffer_duration() {
        let duration = ::std::time::Duration::new(8, 590_000_000);

        assert_eq!(duration, ::acquire::SamplingRate::RATE_1_9kHz.get_buffer_duration());
    }

    #[test]
    fn test_status() {
        let (rx, mut rp) = ::test::create_client();

        rp.acquire.start();
        assert_eq!("ACQ:START\r\n", rx.recv().unwrap());

        rp.acquire.stop();
        assert_eq!("ACQ:STOP\r\n", rx.recv().unwrap());

        rp.acquire.reset();
        assert_eq!("ACQ:RST\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_decimation() {
        let (rx, rp) = ::test::create_client();

        rp.acquire.set_decimation(::acquire::Decimation::DEC_1);
        assert_eq!("ACQ:DEC 1\r\n", rx.recv().unwrap());

        assert_eq!(rp.acquire.get_decimation(), Ok(::acquire::Decimation::DEC_1));
    }

    #[test]
    fn test_average() {
        let (rx, rp) = ::test::create_client();

        rp.acquire.enable_average();
        assert_eq!("ACQ:AVG ON\r\n", rx.recv().unwrap());

        assert_eq!(rp.acquire.is_average_enabled(), true);

        rp.acquire.disable_average();
        assert_eq!("ACQ:AVG OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_gain() {
        let (rx, rp) = ::test::create_client();

        rp.acquire.set_gain(::acquire::Source::IN1, ::acquire::Gain::HV);
        assert_eq!("ACQ:SOUR1:GAIN HV\r\n", rx.recv().unwrap());

        assert_eq!(rp.acquire.get_gain(::acquire::Source::IN1), Ok(::acquire::Gain::HV));
    }
}
