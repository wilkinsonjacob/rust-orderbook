use std::io;

/*
 * Commands to accept:
 * PING     health check
 * ADD      <id> <side> <price> <qty> [LIMIT | IOC | FOK] - default is LIMIT
 * MARKET   <side> <side> <qty>
 * CANCEL   <id>
 * MODIFY   <id> <price> <qty>
 * BBO
 * BOOK
 * QUEUE    <side> <price
 */

enum COMMANDS {
    PING,
    ADD,
    MARKET,
    CANCEL,
    MODIFY,
    BBO,
    BOOK,
    QUEUE,
}

fn main() {
    println!("Input your order command: ");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("error: unable to read user input");
    let command = command.trim();

    if command == "PING" {
        println!("Pong\n");
    }
}
