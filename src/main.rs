/*
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
*/

#![allow(non_snake_case)]

use std::io::{self, Write};
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::HashMap;


fn main() {
    let menuText = String::from(
        "\
0.    Quit.
1.    Check if a word can be made from tiles, including wildcards.
2.    Find the longest word that can be made from given tiles.
====> ",
    );
    let mut wordList: Vec<String> = Vec::new();
    let wordFile = File::open("wordList.txt").unwrap();
    for line in BufReader::new(wordFile).lines() {
        let line = line.unwrap();
        line.trim();
        wordList.push(line);

    }

    'menuLoop: loop {
        clearScreen();

        let mut choice = prompt(&menuText[..], true);

        // Why the hell does this have to be so verbose?
        choice = (&choice[..]).trim().to_string();

        let choice: i8 = choice.parse::<i8>().unwrap_or_else(|e| {

            println!("Please enter a valid number next time.");
            return -1;
        });

        match choice {
            0 => {
                break 'menuLoop;
            }
            1 => {
                let desired = prompt("Please enter a desired word: ", false);
                let curLetters = prompt("Please enter possessed letters (use ? for any wildcards): ",
                                        false);

                let ability = if checkCanMakeWord(&desired, &curLetters) {
                    "can"
                } else {
                    "cannot"
                };

                println!("You {} make {} with {}", ability, desired, curLetters);

            }
            2 => {
                let curLetters = prompt("Please enter possessed letters: ", false);
                let longest = findLongestWord(&curLetters, &wordList);

                println!("The longest word you can make with {} is: {}",
                         curLetters,
                         longest);
            }
            _ => break,
        }
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
    }

}

fn prompt(promptTxt: &str, clearOnEmpty: bool) -> String {
    let mut input = String::new();

    while input == "" {
        if clearOnEmpty {
            clearScreen();
        }
        print!("{}", promptTxt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = (&input[..]).trim().to_string();
    }
    return input;
}

/// My attempt at a cross-platform screen clear-er.
fn clearScreen() {
    let clearCommand = if cfg!(target_os = "linux") {
        "clear"
    } else {
        "cls"
    };

    let output = Command::new(clearCommand).output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn checkCanMakeWord(desired: &String, curLetters: &String) -> bool {
    let mut desLetterCounts: HashMap<char, i32> = HashMap::new();
    let mut curLetterCounts: HashMap<char, i32> = HashMap::new();

    for curChar in desired.chars() {
        // If we haven't started counting that letter yet, set to 0 before incrementing.
        *desLetterCounts.entry(curChar).or_insert(0) += 1;
    }

    for curChar in curLetters.chars() {
        // If we haven't started counting that letter yet, set to 0 before incrementing.
        *curLetterCounts.entry(curChar).or_insert(0) += 1;
    }

    for (letter, count) in desLetterCounts {
        let diff = count - *curLetterCounts.entry(letter).or_insert(-1);
        if diff > 0 {
            if *curLetterCounts.entry('?').or_insert(0) >= diff {
                *curLetterCounts.entry('?').or_insert(0) -= diff;
            } else {
                return false;
            }
        }
    }

    return true;
}

fn findLongestWord(currentTiles: &String, wordList: &Vec<String>) -> String {
    let mut longest = String::new();

    for word in wordList {
        if word.len() > longest.len() {
            if checkCanMakeWord(&word, &currentTiles) {
                longest = word.to_string();
            }
        }
    }

    return longest;
}