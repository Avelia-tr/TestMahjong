use std::{
    io::{self, Read, Write, stdin, stdout},
    thread,
    time::Duration,
};

use crate::visuals_tui::{ansi_codes, input_event::InputEvent, utils::Rawmodder};

const fn b_to_num(b: bool) -> u32 {
    if b { 1 } else { 0 }
}
macro_rules! Args {
    ($name:ident,$($arg:ident),+) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $arg : bool),+
        }

        impl $name {
            pub const fn get_ident_num(&self) -> u32 {
                let mut res = 0;
                $(res <<= 1;res += b_to_num(self.$arg);)+
                res
            }
        }
    };
}

Args!(
    ComprehensiveInputArgs,
    associated_text,
    all_keys_as_escape_codes,
    alternate_keys,
    event_types,
    disambiguate_escape_codes
);

pub struct ComprehensiveInput(());

impl ComprehensiveInput {
    pub fn enable(args: ComprehensiveInputArgs) -> io::Result<Self> {
        let mut out = stdout().lock();

        out.write_all(ansi_codes::EXTENSIVE_INPUT_START)?;
        out.write_all(args.get_ident_num().to_string().as_bytes())?;
        out.write_all(ansi_codes::EXTENSIVE_INPUT_DELEMITER)?;
        out.flush()?;

        Ok(Self(()))
    }

    pub fn read_input(&self, _raw_guard: &Rawmodder) -> io::Result<InputEvent> {
        // [UNSAFE]
        // we hold the guarantees manually
        unsafe { Self::read_input_no_guarantees() }
    }

    unsafe fn read_input_no_guarantees() -> io::Result<InputEvent> {
        let mut buf = vec![];

        let mut input_channel = stdin().lock();

        let mut character = [0u8];

        loop {
            input_channel.read_exact(&mut character)?;
            buf.extend_from_slice(&character);

            if is_terminal(character[0]) {
                break;
            }
        }

        todo!()
        //Ok(InputEvent::parse(buf))
    }

    pub fn read_with_timeout(
        &self,
        dur: Duration,
        _raw_guard: &Rawmodder,
    ) -> Option<Result<InputEvent, io::Error>> {
        // UNSAFE
        // here we hold the guarantees needed for "read input" manually cuz lifetime
        let input_thread = thread::spawn(|| unsafe { Self::read_input_no_guarantees() });
        thread::sleep(dur);

        if input_thread.is_finished() {
            let input = input_thread.join().expect("non panicking function");
            Some(input)
        } else {
            None
        }
    }
}

impl Drop for ComprehensiveInput {
    fn drop(&mut self) {
        let mut out = stdout().lock();

        _ = out.write_all(ansi_codes::EXTENSIVE_INPUT_STOP);
        _ = out.write_all(ansi_codes::EXTENSIVE_INPUT_DELEMITER);

        _ = out.flush();
    }
}

fn is_terminal(c: u8) -> bool {
    c == b'u'
}
