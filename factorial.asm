dbgs "Factorial of 5"

# N
mov $r0 5
# Result
mov $r1 1

loop:
	ble $r0 1 end
	mul $r1 $r1 $r0
	sub $r0 $r0 1
	jmp loop

end:
	dbgs "Result:"
	dbg $r1
