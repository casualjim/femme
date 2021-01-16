//! Print logs as ndjson.

use log::{kv, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::{self, StdoutLock, Write};
use std::time;

/// Start logging.
pub(crate) fn try_start(level: LevelFilter) -> Result<(), SetLoggerError> {
    let logger = Box::new(Logger {});
    let res = log::set_boxed_logger(logger);
    if res.is_ok() {
        log::set_max_level(level)
    }
    res
}

#[derive(Debug)]
pub(crate) struct Logger {}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let level = record.level();
            let time = time::UNIX_EPOCH.elapsed().unwrap().as_millis();
            write!(
                &mut handle,
                "{{\"level\":{},\"time\":{},\"msg\":",
                level.to_string().to_lowercase(), time
            )
            .unwrap();
            serde_json::to_writer(&mut handle, record.args()).unwrap();
            format_kv_pairs(&mut handle, &record);
            writeln!(&mut handle, "}}").unwrap();
        }
    }
    fn flush(&self) {}
}

fn format_kv_pairs<'b>(mut out: &mut StdoutLock<'b>, record: &Record) {
    struct Visitor<'a, 'b> {
        string: &'a mut StdoutLock<'b>,
    }

    impl<'kvs, 'a, 'b> kv::Visitor<'kvs> for Visitor<'a, 'b> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.string, ",\"{}\":\"{}\"", key, val)?;
            Ok(())
        }
    }

    let mut visitor = Visitor { string: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}
