#!/bin/bash
PART="atmega4809"
PROGRAMMER="jtag2updi"
BAUD=115200

if [[ "Z$PORT" == "Z" ]]; then
    echo "Please set \$PORT to your microcontroller location (ex. /dev/ttyACM1)"
    echo "fish: set -x PORT (ls /dev/ttyACM* | tail -n 1)"
    echo "bash: export PORT=\$(ls /dev/ttyACM* | tail -n 1)"
    exit 1
fi


FUSE0="0x00" # Watchdog timer 0x00
FUSE1="0x00" # BOD 0x00
FUSE2="0x00" # OSCCFG 0x02 = 20mhz, 0x01 = 16mhz
FUSE5="0xE4" # SYSCFG0 0xE4
FUSE6="0x02" # SYSCFG1, delay code at startup by x milliseconds. x = 2^(FUSE6)
FUSE7="0x00" # APPEND 0x0
FUSE8="0x00" # BOOTEND 0x0
FUSEA="0xC5" # LOCKBIT 0xC5, must be C5

if [[ "Z$2" == "Zfuse" ]]; then
    FUSEFLAGS="
        -Ufuse0:w:$FUSE0:m \
        -Ufuse1:w:$FUSE1:m \
        -Ufuse2:w:$FUSE2:m \
        -Ufuse5:w:$FUSE5:m \
        -Ufuse6:w:$FUSE6:m \
        -Ufuse7:w:$FUSE7:m \
        -Ufuse8:w:$FUSE8:m"
        # skip for now # -Ufusea:w:$(FUSE0):m 
else
    FUSEFLAGS=""
fi

set -x

# put the microcontroller into a listening state
stty -F "${PORT}" 1200
sleep 0.5

avrdude -v -p$PART -c$PROGRAMMER -P$PORT -b$BAUD \
    -D -Uflash:w:$1:e \
    $FUSEFLAGS
    # skip for now # -Ufusea:w:$(FUSE0):m 
    #-Uflash:w:/tmp/arduino_build_62094/sketch_jan10a.ino.hex:i

set +x
