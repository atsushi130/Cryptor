
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

lazy_static! {
    pub static ref ALPHABETS: Vec<char>           = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ %=!?,.;:*\"\'`@(){}[]<>#$&~^|¥_/\\+-1234567890".chars().collect();
    pub static ref SUBSTITUTION_TABLE1: Vec<char> = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA 0987654321-+\\_/¥|^~&$#><][}{)(@`\'\"*:;.,?!=%".chars().collect();
    pub static ref SUBSTITUTION_TABLE2: Vec<char> = "blsxuvwoqgtpmndzyjrhafekciBLSXUVWOQGTPMNDZYJRHAFEKCI% !=,?;.*:\'\"@`)(}{][><$#~&|^_¥\\/-+2143658709".chars().collect();
    pub static ref SUBSTITUTION_TABLE3: Vec<char> = "lbxsvuowgqptnmzdjyhrfakeicLBXSVUOWGQPTNMZDJYHRFAKEIC9078563412+-/\\¥_^|&~#$<>[]{}()`@\"\':*.;?,=! %".chars().collect();
    pub static ref REFLECTOR: Vec<char>           = "0987654321-+\\/_¥|^~&$#><][}{)(@`\'\"*:;.,?!=% ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba".chars().collect();
    pub static ref PLUGBOARD: Vec<char>           = "%=!?,.;:*\"\'`@(){}[]<>#$&~^|¥/_\\+-1234567890 ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect();
}
