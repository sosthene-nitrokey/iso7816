#![no_main]
use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use iso7816::command::{Command, CommandView};

use std::convert::TryFrom;

#[derive(Arbitrary, Debug)]
struct Input {
    data: Vec<Vec<u8>>,
}

fn test_command<const N: usize>(input: &Input) -> Option<()> {
    if input.data.is_empty() {
        return None;
    }

    let mut acc = Command::<N>::try_from(&*input.data[0]).ok()?;
    assert_eq!(
        acc,
        CommandView::try_from(&*input.data[0])
            .unwrap()
            .to_owned()
            .unwrap()
    );
    for apdu in input.data.iter() {
        let tmp = Command::<N>::try_from(&**apdu).ok()?;
        assert_eq!(
            tmp,
            CommandView::try_from(&**apdu).unwrap().to_owned().unwrap()
        );
        acc.extend_from_command(&tmp).ok();
    }
    None
}

fuzz_target!(|input: Input| {
    test_command::<0>(&input);
    test_command::<1>(&input);
    test_command::<2>(&input);
    test_command::<3>(&input);
    test_command::<4>(&input);
    test_command::<5>(&input);
    test_command::<6>(&input);
    test_command::<7>(&input);
    test_command::<8>(&input);
    test_command::<9>(&input);
    test_command::<10>(&input);
    test_command::<11>(&input);
    test_command::<12>(&input);
    test_command::<13>(&input);
    test_command::<14>(&input);
    test_command::<15>(&input);
    test_command::<16>(&input);
    test_command::<17>(&input);
    test_command::<18>(&input);
    test_command::<19>(&input);
    test_command::<20>(&input);
    test_command::<21>(&input);
    test_command::<22>(&input);
    test_command::<23>(&input);
    test_command::<24>(&input);
    test_command::<25>(&input);
    test_command::<26>(&input);
    test_command::<27>(&input);
    test_command::<28>(&input);
    test_command::<29>(&input);
    test_command::<30>(&input);
    test_command::<31>(&input);
    test_command::<32>(&input);
    test_command::<33>(&input);

    test_command::<126>(&input);
    test_command::<127>(&input);
    test_command::<128>(&input);

    test_command::<255>(&input);
    test_command::<256>(&input);
    test_command::<257>(&input);

    test_command::<1023>(&input);
    test_command::<1024>(&input);
    test_command::<1025>(&input);

    test_command::<2047>(&input);
    test_command::<2048>(&input);
    test_command::<2049>(&input);
});
