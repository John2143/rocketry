/*

0 0x00 RESET X X X X
1 0x02 NMI - Non-Maskable Interrupt from CRC X X X X
2 0x04 VLM - Voltage Level Monitor X X X X
3 0x06 RTC - Overflow or compare match X X X X
4 0x08 PIT - Periodic interrupt X X X X
5 0x0A CCL - Configurable Custom Logic X X X X
6 0x0C PORTA - External interrupt X X X X
7 0x0E TCA0 - Overflow X X X X
8 0x10 TCA0 - Underflow (Split mode) X X X X
9 0x12 TCA0 - Compare channel 0 X X X X
10 0x14 TCA0 - Compare channel 1 X X X X
11 0x16 TCA0 - Compare channel 2 X X X X
12 0x18 TCB0 - Capture X X X X
13 0x1A TCB1 - Capture X X X X
14 0x1C TWI0 - Slave X X X X
15 0x1E TWI0 - Master X X X X
16 0x20 SPI0 - Serial Peripheral Interface 0 X X X X
17 0x22 USART0 - Receive Complete X X X X
18 0x24 USART0 - Data Register Empty X X X X
19 0x26 USART0 - Transmit Complete X X X X
20 0x28 PORTD - External interrupt X X X X
21 0x2A AC0 – Compare X X X X
22 0x2C ADC0 – Result Ready X X X X
23 0x2E ADC0 - Window Compare X X X X
24 0x30 PORTC - External interrupt X X X X
25 0x32 TCB2 - Capture X X X X
26 0x34 USART1 - Receive Complete X X X X
27 0x36 USART1 - Data Register Empty X X X X
28 0x38 USART1 - Transmit Complete X X X X
29 0x3A PORTF - External interrupt X X X X
30 0x3C NVM - Ready X X X X
31 0x3E USART2 - Receive Complete X X X X
32 0x40 USART2 - Data Register Empty X X X X
33 0x42 USART2 - Transmit Complete X X X X
34 0x44 PORTB - External interrupt X
35 0x46 PORTE - External interrupt X X
36 0x48 TCB3 - Capture X X
37 0x4A USART3 - Receive Complete X X
38 0x4C USART3 - Data Register Empty X X
39 0x4E USART3 - Transmit Complete X X
*/

//i2c master write
#[no_mangle]
pub fn __vector_15() {
    todo!()
}

#[no_mangle]
pub fn __vector_39() {
    todo!()
}
