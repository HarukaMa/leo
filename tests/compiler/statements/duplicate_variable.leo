/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

program test.aleo {    
    function main(k: bool) -> bool {
    	let x: u8 = 1u8;
      	let x: bool = true;
    	
    	return k == true;
    }}
