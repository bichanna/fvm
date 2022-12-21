#[macro_export]
macro_rules! report_error_with_lines {
    ($self: expr, $t: expr, $message: expr) => {
        $self.err_manager.create_and_add_error($message, $t.1, $t.2)
    };
}
