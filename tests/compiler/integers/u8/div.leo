/*
namespace: Compile
expectation: Pass
input_file: inputs/div.in
*/

program test.aleo {    
    transition main(a: u8, b: u8, c: u8) -> bool {
        return a / b == c;
    }
}
