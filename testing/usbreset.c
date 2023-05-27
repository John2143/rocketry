//https://askubuntu.com/questions/645/how-do-you-reset-a-usb-device-from-the-command-line
/* usbreset -- send a USB port reset to a USB device */

#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <sys/ioctl.h>

#include <linux/usbdevice_fs.h>

// this is a c version of this function:
// https://github.com/arduino/Arduino/blob/89539b1131f8cde9f7a83225f21c811071af53a8/arduino-core/src/processing/app/Serial.java#L98
/*

public static boolean touchForCDCReset(String iname) throws SerialException {
    SerialPort serialPort = new SerialPort(iname);
    try {
        serialPort.openPort();
        serialPort.setParams(1200, 8, SerialPort.STOPBITS_1, SerialPort.PARITY_NONE);
        serialPort.setDTR(false);
        serialPort.closePort();
        return true;
    } catch (SerialPortException e) {
        throw new SerialException(format(tr("Error touching serial port ''{0}''."), iname), e);
    } finally {
        if (serialPort.isOpened()) {
            try {
                serialPort.closePort();
            } catch (SerialPortException e) {
                // noop
            }
        }
    }
}
*/

int main(int argc, char **argv)
{
    const char *filename;
    int fd;
    int rc;

    if (argc != 2) {
        fprintf(stderr, "Usage: usbreset device-filename\n");
        return 1;
    }
    filename = argv[1];

    fd = open(filename, O_RDWR);
    if (fd < 0) {
        perror("Error opening output file");
        return 1;
    }

    if(1){
        /*printf("Resetting USB device %s\n", filename);*/
        /*rc = ioctl(fd, USBDEVFS_RESET, 0);*/
        /*if (rc < 0) {*/
            /*perror("Error in ioctl");*/
            /*return 1;*/
        /*}*/
        /*printf("Reset successful\n");*/

        // Set DTR to false
        int returnValue = 0;
        int lineStatus;
        ioctl(fd, TIOCMGET, &lineStatus);
        lineStatus &= ~TIOCM_DTR;
        rc = ioctl(fd, TIOCMSET, &lineStatus);

        if (rc != 0) {
            perror("Error in ioctl TIOCM");
            return 1;
        }
    }
    close(fd);

    return 0;
}
