// The handshake used by the card and the door involves an operation that transforms a subject
// number. To transform a subject number, start with the value 1. Then, a number of times called
// the loop size, perform the following steps:
//
// 1. Set the value to itself multiplied by the subject number.
// 2. Set the value to the remainder after dividing the value by 20201227.
//
// The card always uses a specific, secret loop size when it transforms a subject number. The door
// always uses a different, secret loop size.
//
// The cryptographic handshake works like this:
//
// The card transforms the subject number of 7 according to the card's secret loop size. The result
// is called the card's public key.
//
// The door transforms the subject number of 7 according to the door's secret loop size. The result
// is called the door's public key.
//
// The card and door use the wireless RFID signal to transmit the two public keys (your puzzle
// input) to the other device. Now, the card has the door's public key, and the door has the card's
// public key. Because you can eavesdrop on the signal, you have both public keys, but neither
// device's loop size.
//
// The card transforms the subject number of the door's public key according to the card's loop
// size. The result is the encryption key.
//
// The door transforms the subject number of the card's public key according to the door's loop
// size. The result is the same encryption key as the card calculated.

const SUBJECT_NUMBER: usize = 7;
const MODULO: usize = 20201227;
const PUBLIC_KEYS: [usize; 2] = [14788856, 19316454];

fn get_loop_size(public_key: usize) -> usize {
    let mut loop_size: usize = 0;
    let mut value = 1;
    loop {
        if value == public_key {
            return loop_size;
        }
        value *= SUBJECT_NUMBER;
        value %= MODULO;
        loop_size += 1;
    }
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= MODULO;
    }
    return value;
}

pub fn part1() -> usize {
    let loopsize = dbg!(get_loop_size(PUBLIC_KEYS[0]));
    let enc_key = transform(PUBLIC_KEYS[1], loopsize);
    return enc_key;
}
