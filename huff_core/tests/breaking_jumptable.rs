use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_breaking_jump_table() {
    let source: &str = r#"
        #define jumptable__packed TEST_JUMPTABLE {
            label_0_0 label_0_1 label_0_2 label_0_3 label_0_4 label_0_5 label_0_6 label_0_7 label_0_8 label_0_9 label_0_a label_0_b label_0_c label_0_d label_0_e label_0_f
            label_1_0 label_1_1 label_1_2 label_1_3 label_1_4 label_1_5 label_1_6 label_1_7 label_1_8 label_1_9 label_1_a label_1_b label_1_c label_1_d label_1_e label_1_f
            label_2_0 label_2_1 label_2_2 label_2_3 label_2_4 label_2_5 label_2_6 label_2_7 label_2_8 label_2_9 label_2_a label_2_b label_2_c label_2_d label_2_e label_2_f
            label_3_0 label_3_1 label_3_2 label_3_3 label_3_4 label_3_5 label_3_6 label_3_7 label_3_8 label_3_9 label_3_a label_3_b label_3_c label_3_d label_3_e label_3_f
            label_4_0 label_4_1 label_4_2 label_4_3 label_4_4 label_4_5 label_4_6 label_4_7 label_4_8 label_4_9 label_4_a label_4_b label_4_c label_4_d label_4_e label_4_f
            label_5_0 label_5_1 label_5_2 label_5_3 label_5_4 label_5_5 label_5_6 label_5_7 label_5_8 label_5_9 label_5_a label_5_b label_5_c label_5_d label_5_e label_5_f
            label_5_0 label_5_1 label_5_2 label_5_3 label_5_4 label_5_5 label_5_6 label_5_7 label_5_8 label_5_9 label_5_a label_5_b label_5_c label_5_d label_5_e label_5_f
            label_5_0 label_5_1 label_5_2 label_5_3 label_5_4 label_5_5 label_5_6 label_5_7 label_5_8 label_5_9 label_5_a label_5_b label_5_c label_5_d label_5_e
        }


        // Program Entrypoint
        #define macro MAIN() = takes(0) returns (0) {
            // Copy the jumptable to the stack
            __tablesize(TEST_JUMPTABLE) __tablestart(TEST_JUMPTABLE) 0x0 codecopy

            // Revert here if the labels jump back
            0x00 dup1 revert

            // Define labels
            label_0_0: dup1
            label_0_1: dup1
            label_0_2: dup1
            label_0_3: dup1
            label_0_4: dup1
            label_0_5: dup1
            label_0_6: dup1
            label_0_7: dup1
            label_0_8: dup1
            label_0_9: dup1
            label_0_a: dup1
            label_0_b: dup1
            label_0_c: dup1
            label_0_d: dup1
            label_0_e: dup1
            label_0_f: dup1

            label_1_0: dup1
            label_1_1: dup1
            label_1_2: dup1
            label_1_3: dup1
            label_1_4: dup1
            label_1_5: dup1
            label_1_6: dup1
            label_1_7: dup1
            label_1_8: dup1
            label_1_9: dup1
            label_1_a: dup1
            label_1_b: dup1
            label_1_c: dup1
            label_1_d: dup1
            label_1_e: dup1
            label_1_f: dup1

            label_2_0: dup1
            label_2_1: dup1
            label_2_2: dup1
            label_2_3: dup1
            label_2_4: dup1
            label_2_5: dup1
            label_2_6: dup1
            label_2_7: dup1
            label_2_8: dup1
            label_2_9: dup1
            label_2_a: dup1
            label_2_b: dup1
            label_2_c: dup1
            label_2_d: dup1
            label_2_e: dup1
            label_2_f: dup1

            label_3_0: dup1
            label_3_1: dup1
            label_3_2: dup1
            label_3_3: dup1
            label_3_4: dup1
            label_3_5: dup1
            label_3_6: dup1
            label_3_7: dup1
            label_3_8: dup1
            label_3_9: dup1
            label_3_a: dup1
            label_3_b: dup1
            label_3_c: dup1
            label_3_d: dup1
            label_3_e: dup1
            label_3_f: dup1

            label_4_0: dup1
            label_4_1: dup1
            label_4_2: dup1
            label_4_3: dup1
            label_4_4: dup1
            label_4_5: dup1
            label_4_6: dup1
            label_4_7: dup1
            label_4_8: dup1
            label_4_9: dup1
            label_4_a: dup1
            label_4_b: dup1
            label_4_c: dup1
            label_4_d: dup1
            label_4_e: dup1
            label_4_f: dup1

            label_5_0: dup1
            label_5_1: dup1
            label_5_2: dup1
            label_5_3: dup1
            label_5_4: dup1
            label_5_5: dup1
            label_5_6: dup1
            label_5_7: dup1
            label_5_8: dup1
            label_5_9: dup1
            label_5_a: dup1
            label_5_b: dup1
            label_5_c: dup1
            label_5_d: dup1
            label_5_e: dup1
            label_5_f: dup1
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the main macro bytecode
    let mbytes = Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None).unwrap();
    assert_eq!(mbytes, String::from("60fe6100ca5f395f80fd5b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b805b80000a000c000e00100012001400160018001a001c001e00200022002400260028002a002c002e00300032003400360038003a003c003e00400042004400460048004a004c004e00500052005400560058005a005c005e00600062006400660068006a006c006e00700072007400760078007a007c007e00800082008400860088008a008c008e00900092009400960098009a009c009e00a000a200a400a600a800aa00ac00ae00b000b200b400b600b800ba00bc00be00c000c200c400c600c800aa00ac00ae00b000b200b400b600b800ba00bc00be00c000c200c400c600c800aa00ac00ae00b000b200b400b600b800ba00bc00be00c000c200c400c6"));
}
