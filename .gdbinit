target remote :3333

monitor reset init
monitor arm semihosting enable
monitor tpiu config internal /tmp/itm.log uart off 84000000
monitor itm port 0 on
load
monitor reset init
