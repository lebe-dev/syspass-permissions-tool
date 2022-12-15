pub mod logging {
    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::append::rolling_file::RollingFileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log::LevelFilter;

    const ROLLING_APPENDER_NAME: &str = "rolling";

    pub const LOG_FILE_PATH: &str = "spt.log";

    fn get_logging_level_from_string(level: &str) -> LevelFilter {
        return match level {
            "debug" => LevelFilter::Debug,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "trace" => LevelFilter::Trace,
            "off" => LevelFilter::Off,
            _ => LevelFilter::Info
        };
    }

    pub fn get_logging_config(logging_level: &str) -> Config {
        let level = get_logging_level_from_string(logging_level);

        Config::builder()
            .appender(get_rolling_appender())
            .logger(get_default_logger())
            .build(
            Root::builder()
                .appender(ROLLING_APPENDER_NAME)
                .build(level)
            ).expect(&format!("unable to create log file '{}'", LOG_FILE_PATH))
    }

    fn get_rolling_appender() -> Appender {
        let fixed_window_roller = FixedWindowRoller::builder()
                                        .build("spt.log.{}", 3).unwrap();
        let size_trigger = SizeTrigger::new(100_000_000);
        let policy = CompoundPolicy::new(
            Box::new(size_trigger), Box::new(fixed_window_roller)
        );
        let rolling_appender = RollingFileAppender::builder()
            .encoder(get_encoder())
            .build("spt.log", Box::new(policy)).unwrap();

        Appender::builder()
            .build(ROLLING_APPENDER_NAME, Box::new(rolling_appender))
    }

    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - {m}{n}"))
    }

    fn get_default_logger() -> Logger {
        Logger::builder()
                .build("default", LevelFilter::Info)
    }
}
