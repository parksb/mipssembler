pub const INPUT_CASE_1: &str = "
	.data
data1:	.word	100
data2:	.word	200
data3:	.word	0x12345678
	.text
main:
	and	$17, $17, $0
	and	$18, $18, $0
	la	$8, data1
	la	$9, data2
	and	$10, $10, $0
lab1:
	and	$11, $11, $0
lab2:
	addiu	$17, $17, 0x1
	addiu	$11, $11, 0x1
	or	$9, $9, $0
	bne	$11, $8, lab2
lab3:
	addiu	$18, $18, 0x2
	addiu	$11, $11, 1
	sll	$18, $17, 1
	srl	$17, $18, 1
	and	$19, $17, $18
	bne	$11, $9, lab3
lab4:
	addu	$5, $5, $31
	nor	$16, $17, $18
	beq	$10, $8, lab5
	j	lab1
lab5:
	ori	$16, $16, 0xf0f0
";

pub const OUTPUT_CASE_1: &str = "000000000000000000000000010110000000000000000000000000000000110000000010001000001000100000100100000000100100000010010000001001000011110000001000000100000000000000111100000010010001000000000000001101010010100100000000000001000000000101000000010100000010010000000001011000000101100000100100001001100011000100000000000000010010010101101011000000000000000100000001001000000100100000100101000101010110100011111111111111000010011001010010000000000000001000100101011010110000000000000001000000000001000110010000010000000000000000010010100010000100001000000010001100101001100000100100000101010110100111111111111110100000000010111111001010000010000100000010001100101000000000100111000100010100100000000000000000010000100000010000000000000000011000110110000100001111000011110000000000000000000000000000011001000000000000000000000000001100100000010010001101000101011001111000";
