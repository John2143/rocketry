#!/bin/bash
PART="atmega4809"
PROGRAMMER="jtag2updi"
#BAUD=9600
#BAUD=57600
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
FUSE5="0xC8" # SYSCFG0 0xE4, dont forget to pullup reset
FUSE6="0x02" # SYSCFG1, delay code at startup by x milliseconds. x = 2^(FUSE6)
FUSE7="0x00" # APPEND 0x0
FUSE8="0x00" # BOOTEND 0x0
FUSEA="0xC5" # LOCKBIT 0xC5, must be C5

USB_RESET=0
SCREEN_BAUD=0

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

elif [[ "Z$2" == "Zreset" ]]; then
    USB_RESET=1
    FUSEFLAGS=""
else
    FUSEFLAGS=""
fi

if [[ "Z$3" != "Z" ]]; then
    SCREEN_BAUD=$3
fi

set -x

if [[ $USB_RESET > 0 ]]; then
    stty -F "${PORT}" 1200
    sudo "../testing/usbreset" "${PORT}"

    # in fish, () && set -x PORT () && echo $PORT
    #ARDUINO=$(lsusb | grep duino | choose 1 3 | sed "s/://" | tr " " "/")
    #sudo "../testing/usbreset" "/dev/bus/usb/$ARDUINO"

    #sleep .1
    #PORT=$(ls /dev/ttyACM* | tail -n 1)
    #echo "new port is $PORT"
fi

while :; do
    sleep .5
    [ -c "${PORT}" ] && break
done

avrdude -v -p$PART -c$PROGRAMMER -P$PORT -b$BAUD \
    $FUSEFLAGS \
    -D -e -Uflash:w:$1:e
    # skip for now # -Ufusea:w:$(FUSE0):m 
    #-Uflash:w:/tmp/arduino_build_62094/sketch_jan10a.ino.hex:i

# avrdude v -patmega4809 -cjtag2updi -P/dev/ttyACM1 -b115200 -e -D -Uflash:w:/tmp/arduino_build_365595/Blink.ino.hex:i -Ufuse2:w:0x01:m -Ufuse5:w:0xC9:m -Ufuse8:w:0x00:m {upload.extra_files} 

if [[ $SCREEN_BAUD > 0 ]]; then
    minicom $PORT -b $SCREEN_BAUD
fi

#screen $PORT 9600 8n1
set +x
