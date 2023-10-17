dbg "Factorial of 5"

# N
mov 5 r0
# Result
mov 1 r1

loop:
	ble r0 1 end
	mul r1 r1 r0
	sub r0 r0 1
	jmp loop

end:
	dbg "Result:"
	dbg r1
