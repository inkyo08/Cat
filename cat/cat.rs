pub use cor::*;

#[cfg(test)]
mod tests {
    use cor::log::*;

    #[test]
    fn test_log_levels() {
        set_level(LOG_TRACE);
        
        log_trace(file!(), line!(), "This is a trace message");
        log_debug(file!(), line!(), "This is a debug message");
        log_info(file!(), line!(), "This is an info message");
        log_warn(file!(), line!(), "This is a warning message");
        log_error(file!(), line!(), "This is an error message");
        log_fatal(file!(), line!(), "This is a fatal message");
        
        // Test level filtering
        set_level(LOG_WARN);
        log_info(file!(), line!(), "This should be hidden");
        log_warn(file!(), line!(), "This should be visible");
        log_error(file!(), line!(), "This should also be visible");
    }
}