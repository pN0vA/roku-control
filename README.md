# roku-control
A rust tool accessing the roku api for external control

## Installation
Building from source:

git clone https://github.com/pN0vA/roku-control.git

cd roku-control/

cargo run

cd /target/debug/

./roku-control -h or roku-control.exe -h

## Usage
roku-control.exe -t [Roku Device IP] -p [port] -s [query/keypress] -c poweroff

## Commands
-t the roku device target

-p port of roku target usually 8060

-s subdomain of Roku api [Query or Keypress]

-c command to send to roku device

-L will list all commands allotted for each command in the api Query/Keypress

### Only Works on Linux with Ncat and Wireshark Installed
-l will create "roku_request.txt" and ncat to start listening for roku devices then, starts up wireshark with the correct filter for the devices.
 the filter will not work right away on wireshark as you for now will have to copy the filter pick your listening device then put it back as a filter and press enter.

## Working on
- [ ] Making a functional reverse shell

- [ ] Bug Fixes
