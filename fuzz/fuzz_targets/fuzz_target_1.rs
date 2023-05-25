#![no_main]

use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use iso7816::command::{class, Command, CommandBuilder, CommandView};

use std::iter::repeat;

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    class: u8,
    instruction: u8,
    p1: u8,
    p2: u8,
    le: u16,
    buf_len: usize,
    buf_lens: Vec<usize>,
    data: &'a [u8],
}

fuzz_target!(|data: Input| {
    if data.class == 0b11101111 {
        // pathological class that can't be chained because it makes it a 0xFF
        return;
    }
    let Ok(class) = class::Class::try_from(data.class) else {
        return;
    };
    let ins = data.instruction.into();

    let buffer = &mut [0; 4096][..data.buf_len.min(4096).max(128)];

    let command = CommandBuilder::new(class, ins, data.p1, data.p2, data.data, data.le);
    match command.clone().serialize_into(buffer) {
        Ok(len) => {
            // dbg!(&buffer[..len][..len]);
            let view = CommandView::try_from(&buffer[..len]).unwrap();
            assert_eq!(view, command, "buffer: {:02x?}", &buffer[..len]);
        }
        Err((len, mut rem)) => {
            // dbg!(&buffer[..len]);
            let mut parsed = Command::<4096>::try_from(&buffer[..len]).unwrap();
            // Loop with arbitrary buflens forever
            for buflen in repeat(data.buf_lens.iter().chain([&128])).flatten() {
                let buffer = &mut [0; 4096][..(*buflen).min(4096).max(128)];
                match rem.serialize_into(buffer) {
                    Ok(len) => {
                        // dbg!(&buffer[..len]);
                        let view = CommandView::try_from(&buffer[..len]).unwrap();
                        parsed.extend_from_command_view(view).unwrap();
                        assert_eq!(command, parsed.as_view());
                        return;
                    }
                    Err((len, new_rem)) => {
                        // dbg!(&buffer[..len]);
                        rem = new_rem;

                        let view = CommandView::try_from(&buffer[..len]).unwrap();
                        parsed.extend_from_command_view(view).unwrap();
                    }
                }
            }
        }
    }
    // fuzzed code goes here
});