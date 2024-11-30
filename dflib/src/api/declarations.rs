#[macro_export]
macro_rules! headers {
    (
        $(PlayerEvent::$event_name:ident => fn $event_code:expr;)*
        $(EntityEvent::$ent_event_name:ident => fn $ent_event_code:expr;)*
        $(Function::$function_name:ident => fn $fn_code:expr;)*
        $(Process::$process_name:ident => fn $proc_code:expr;)*
    ) => {
        fn main() {
            $(rustfire::api::event::PlayerEvent::$event_name($event_code);)*
            $(rustfire::api::event::EntityEvent::$ent_event_name($ent_event_code);)*
            $(rustfire::api::event::Functions::declare(stringify!($function_name), $fn_code);)*
            $(rustfire::api::event::Processes::declare(stringify!($process_name), $proc_code);)*
            rustfire::api::done();
        }
    };
}